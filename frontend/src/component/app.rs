use crate::component::*;
use gloo::console::log;
use yew::prelude::*;

pub struct App {
    profit_loss_update_counter: i64,
}

pub enum Msg {
    UpdateProfitLoss(bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            profit_loss_update_counter: 0,
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                // Account for transaction list props
                <transaction_list::TransactionList update={ctx.link().callback(|should_update| Msg::UpdateProfitLoss(should_update))} />
                <profit_loss::ProfitLoss update_counter={
                    {
                        log!(format!("should_update_profit_loss in fn view(): {}", self.profit_loss_update_counter));
                    }
                    self.profit_loss_update_counter
                }/>
                <add_transaction_overlay::AddTransactionOverlay />
            </main>
        }
    }
}
