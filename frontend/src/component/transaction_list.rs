use gloo::console::info;
use gloo::console::log;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use self::_Props::should_update;

pub struct TransactionList {
    transactions: Vec<Transaction>,
    item_name: String,
    should_update: bool,
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
    pub search_string: String,
    pub should_update: i64,
    pub update: Callback<bool>,
}

impl Component for TransactionList {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let resp = Request::get("http://localhost:43211/api/v1/trade")
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
            should_update: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        log!(format!("props.update: {}", old_props.should_update));
        self.item_name = ctx.props().search_string.clone();
        ctx.link().send_message(Msg::Search);
        if ctx.props().should_update != old_props.should_update {
            self.should_update = true;
        }
        true
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
                    let mut url = String::from("http://localhost:43211/api/v1/trade");
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

                if self.should_update {
                    ctx.props().update.emit(self.should_update);
                }

                self.should_update  = false;

                true
            },
            Msg::DeleteTransaction(id) => {
                // Send DELETE request to backend
                ctx.link().send_future(async move {
                    let url = format!("http://localhost:43211/api/v1/trade?id={}", id);
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

                self.should_update = true;
                
                true
            },
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="transaction-panel">
                    <div class="transaction-list-container">
                        <table class="transaction-list-table">
                            <thead>
                                <tr>
                                    <th style="border-top-left-radius:8px">{ "ID" }</th>
                                    <th>{ "Name" }</th>
                                    <th>{ "Quantity" }</th>
                                    <th>{ "Price" }</th>
                                    <th>{ "Sale or Purchase" }</th>
                                    <th>{ "Date" }</th>
                                    <th style="border-top-right-radius:8px">{ "Actions" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { self.transactions.iter().enumerate().map(|(index, transaction)| self.render_row(ctx, index, transaction)).collect::<Vec<VNode>>() }
                            </tbody>
                        </table>
                    </div>
                </div>
            </>
        }
    }
}

impl TransactionList {
    fn render_row(&self, ctx: &Context<Self>, index: usize, transaction: &Transaction) -> Html {
        let last_row_style = if index == self.transactions.len() - 1 {
            "border-bottom:0px"
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
                <td style={ last_row_style }> { &transaction.id }</td>
                <td style={ last_row_style }> { &transaction.item_name }</td>
                <td style={ last_row_style }> { transaction.quantity }</td>
                <td style={ last_row_style }> { formatted_total_price }</td>
                <td style={ last_row_style }> {
                    if transaction.is_purchase {
                        "Purchase"
                    } else {
                        "Sale"
                    }
                }</td>
                <td style={ last_row_style }> { &transaction.timestamp }</td>
                <td style={ last_row_style }> { 
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
