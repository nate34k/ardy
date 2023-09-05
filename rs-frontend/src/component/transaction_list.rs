use serde::{Serialize, Deserialize};
use yew::prelude::*;

pub struct TransactionList {
    transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub quantity: i64,
    pub price: i64,
    pub sale_or_purchase: String,
    pub date: String,
}

impl Component for TransactionList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            transactions: vec![
                Transaction {
                    id: 1,
                    name: "Noxious Scythe".to_string(),
                    quantity: 1,
                    price: 300_000_000,
                    sale_or_purchase: "Sale".to_string(),
                    date: "2021-01-01".to_string(),
                },
                Transaction {
                    id: 2,
                    name: "Noxious Scythe".to_string(),
                    quantity: 1,
                    price: 320_000_000,
                    sale_or_purchase: "Purchase".to_string(),
                    date: "2021-01-01".to_string(),
                },
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
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
                <td style={ last_row_style_left }>{ transaction.id }</td>
                <td>{ &transaction.name }</td>
                <td>{ transaction.quantity }</td>
                <td>{ transaction.price }</td>
                <td>{ &transaction.sale_or_purchase }</td>
                <td style={ last_row_style_right }>{ &transaction.date }</td>
            </tr>
        }
    }
}
