use crate::component::*;
use gloo::console::log;
use yew::{prelude::*, platform::time::sleep};

pub struct App {
    should_update_profit_loss: bool,
}

pub enum Msg {
    UpdateProfitLoss(bool),
    ResetUpdateProfitLoss,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            should_update_profit_loss: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProfitLoss(should_update) => {
                log!(format!("update: {}", should_update));

                // Update profit loss
                self.should_update_profit_loss = should_update;

                log!(format!("should_update_profit_loss: {}", self.should_update_profit_loss));

                // Reset update after 1 second
                ctx.link().send_future(async {
                    let duration = std::time::Duration::from_secs(1);
                    sleep(duration).await;
                    Msg::ResetUpdateProfitLoss
                });

                true
            },
            Msg::ResetUpdateProfitLoss => {
                self.should_update_profit_loss = false;

                log!(format!("should_update_profit_loss: {}", self.should_update_profit_loss));

                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                // Account for transaction list props
                <transaction_list::TransactionList update={ctx.link().callback(|should_update| Msg::UpdateProfitLoss(should_update))} />
                <profit_loss::ProfitLoss update={self.should_update_profit_loss}/>
                <add_transaction_overlay::AddTransactionOverlay />
            </main>
        }
    }
}
