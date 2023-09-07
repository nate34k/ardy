use gloo::console::info;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct TransactionList {
    transactions: Vec<Transaction>,
    item_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub item_name: String,
    pub quantity: i64,
    pub total_price: i64,
    pub is_purchase: bool,
    pub timestamp: String,
}

pub enum Msg {
    UpdateItemName(String),
    Search,
    GetTransactionsComplete(Vec<Transaction>),
}

impl Component for TransactionList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let resp = Request::get("http://localhost:5000/api/v1/trade")
                .send()
                .await;

            match resp {
                Ok(resp) => {
                    let transactions = resp.json::<Vec<Transaction>>().await.unwrap();
                    Msg::GetTransactionsComplete(transactions)
                },
                Err(_) => {
                    Msg::GetTransactionsComplete(Vec::new())
                },
            }
        });

        Self { 
            transactions: Vec::new(),
            item_name: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateItemName(name) => {
                self.item_name = name;
                false
            },
            Msg::Search => {
                let item_name = self.item_name.clone();
                // Send request to backend
                ctx.link().send_future(async move {
                    let mut url = String::from("http://localhost:5000/api/v1/trade");
                    if !item_name.is_empty() {
                        url.push_str(&format!("?item_name={}", item_name));
                    }
                    let resp = Request::get(&url)
                        .send()
                        .await;

                    match resp {
                        Ok(resp) => {
                            let transactions = resp.json::<Vec<Transaction>>().await.unwrap();
                            Msg::GetTransactionsComplete(transactions)
                        },
                        Err(_) => {
                            Msg::GetTransactionsComplete(Vec::new())
                        },
                    }
                });
                true
            },
            Msg::GetTransactionsComplete(transactions) => {
                info!(format!("Got transactions: {:?}", transactions));
                self.transactions = transactions;
                true
            },
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="search-bar">
                    <input
                        type="text"
                        placeholder="Enter item name"
                        value={self.item_name.clone()}
                        oninput = {
                            ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateItemName(input.value())
                            })
                        }
                    />
                    <button onclick={ctx.link().callback(|_| Msg::Search)}>
                        {"Search"}
                    </button>
                </div>
                <div>
                    <h1>{ "Transactions" }</h1>
                    <table class="transaction-list-table">
                        <thead>
                            <tr>
                                <th style="border-top-left-radius:10px">{ "ID" }</th>
                                <th>{ "Name" }</th>
                                <th>{ "Quantity" }</th>
                                <th>{ "Price" }</th>
                                <th>{ "Sale or Purchase" }</th>
                                <th style="border-top-right-radius:10px">{ "Date" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for self.transactions.iter().enumerate().map(|(index, transaction)| self.render_row(index, transaction)) }
                        </tbody>
                    </table>
                </div>
            </>
        }
    }
}

impl TransactionList {
    fn render_row(&self, index: usize, transaction: &Transaction) -> Html {
        let last_row_style_left = if index == self.transactions.len() - 1 {
            "border-bottom-left-radius:10px"
        } else {
            ""
        };

        let last_row_style_right = if index == self.transactions.len() - 1 {
            "border-bottom-right-radius:10px"
        } else {
            ""
        };
    
        html! {
            <tr>
                <td style={ last_row_style_left }>{ "1" }</td>
                <td>{ &transaction.item_name }</td>
                <td>{ transaction.quantity }</td>
                <td>{ transaction.total_price }</td>
                <td>{ &transaction.is_purchase }</td>
                <td style={ last_row_style_right }>{ &transaction.timestamp }</td>
            </tr>
        }
    }
}