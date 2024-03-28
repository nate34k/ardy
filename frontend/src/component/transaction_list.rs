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
    pub id: i64,
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
    DeleteTransaction(i64),
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub update: Callback<bool>,
}

impl Component for TransactionList {
    type Message = Msg;
    type Properties = Props;

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
                self.transactions = transactions;
                // Send callback to profit_loss component to update profit_loss
                ctx.props().update.emit(true);
                true
            },
            Msg::DeleteTransaction(id) => {
                // Send DELETE request to backend
                ctx.link().send_future(async move {
                    let url = format!("http://localhost:5000/api/v1/trade?id={}", id);
                    let resp = Request::delete(&url)
                        .send()
                        .await;

                    match resp {
                        Ok(_) => {
                            Msg::Search
                        },
                        Err(_) => {
                            Msg::Search
                        },
                    }
                });
                
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
                                <th>{ "Date" }</th>
                                <th style="border-top-right-radius:10px">{ "Actions" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for self.transactions.iter().enumerate().map(|(index, transaction)| self.render_row(ctx, index, transaction)) }
                        </tbody>
                    </table>
                </div>
            </>
        }
    }
}

impl TransactionList {
    fn render_row(&self, ctx: &Context<Self>, index: usize, transaction: &Transaction) -> Html {
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

        let id = transaction.id.clone();

        fn format_with_commas(n: i64) -> String {
            let is_negative = n < 0;
            let mut s = n.abs().to_string();
            let mut pos = s.len() as isize - 3;
            
            while pos > 0 {
                s.insert(pos as usize, ',');
                pos -= 3;
            }
            
            if is_negative {
                s = format!("({})", s);
            }
            
            s
        }

        let formatted_total_price = format_with_commas(transaction.total_price);
    
        html! {
            <tr>
                <td style={ last_row_style_left }> { &transaction.id }</td>
                <td>{ &transaction.item_name }</td>
                <td>{ transaction.quantity }</td>
                <td>{ formatted_total_price }</td>
                <td> {
                    if transaction.is_purchase {
                        "Purchase"
                    } else {
                        "Sale"
                    }
                }</td>
                <td>{ &transaction.timestamp }</td>
                <td style={ last_row_style_right }>{ 
                    html! {
                        <>
                            <button class="material-icons" onclick={ctx.link().callback(move |_| Msg::DeleteTransaction(id))}> {
                                "delete"
                            }
                            </button>
                        </>
                    }
                 }</td>
            </tr>
        }
    }
}
