use crate::component::*;
use gloo::console::log;
use yew::prelude::*;

pub struct App {
    should_update_profit_loss: bool,
}

pub enum Msg {
    UpdateProfitLoss(bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            should_update_profit_loss: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProfitLoss(should_update) => {
                // Update profit loss
                if should_update {
                    log!("Updating profit loss");
                    self.should_update_profit_loss = !self.should_update_profit_loss;
                } else {
                    log!("Not updating profit loss");
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
                <profit_loss::ProfitLoss update={self.should_update_profit_loss}/>
                <add_transaction_overlay::AddTransactionOverlay />
            </main>
        }
    }
}
