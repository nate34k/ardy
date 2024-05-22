use crate::component::*;
use gloo::console::log;
use yew::prelude::*;

pub struct App {
    profit_loss_update_counter: i64,
    transaction_list_update_counter: i64,
    search_string: String,
}

pub enum Msg {
    UpdateProfitLoss(bool),
    UpdateTransactionList(bool),
    UpdateTransactionListSearch(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            profit_loss_update_counter: 0,
            transaction_list_update_counter: 0,
            search_string: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProfitLoss(should_update) => {
                log!(format!("should_update in fn update(): {}", should_update));

                // Update profit loss
                if should_update {
                    self.profit_loss_update_counter += 1;
                }

                true
            },
            Msg::UpdateTransactionList(should_update) => {
                log!(format!("should_update in fn update(): {}", should_update));

                // Update transaction list
                if should_update {
                    self.transaction_list_update_counter += 1;
                    self.profit_loss_update_counter += 1;
                }

                true
            },
            Msg::UpdateTransactionListSearch(item_name) => {
                log!(format!("item_name in fn update(): {}", item_name));

                // Update search string
                self.search_string = item_name;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="base" class="pattern">
                <main>
                    // Search bar
                    <search_bar::SearchBar
                        on_search={
                            ctx.link().callback(|search_string| Msg::UpdateTransactionListSearch(search_string))
                        }/>

                    // Account for transaction list props
                    <transaction_list::TransactionList should_update={
                        {
                            log!(format!("should_update_transaction_list in fn view(): {}", self.transaction_list_update_counter));
                        }
                        self.transaction_list_update_counter
                    } search_string={
                        {
                            log!(format!("search_string in fn view(): {}", self.search_string));
                        }
                        self.search_string.clone()
                    } update={ctx.link().callback(|should_update| Msg::UpdateProfitLoss(should_update))} />
                    <profit_loss::ProfitLoss update_counter={
                        {
                            log!(format!("should_update_profit_loss in fn view(): {}", self.profit_loss_update_counter));
                        }
                        self.profit_loss_update_counter
                    }/>
                    <add_transaction_overlay::AddTransactionOverlay update={ctx.link().callback(|should_update| Msg::UpdateTransactionList(should_update))} />
                </main>
            </div>
        }
    }
}
