use gloo::console::info;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

pub struct TransactionList {
    transactions: Vec<Transaction>,
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

        Self { transactions: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetTransactionsComplete(transactions) => {
                info!(format!("Got transactions: {:?}", transactions));
                self.transactions = transactions;
                true
            },
        };

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
