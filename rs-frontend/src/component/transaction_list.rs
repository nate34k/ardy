use yew::prelude::*;

pub struct TransactionList;

impl Component for TransactionList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // TODO: Fetch and display the transactions from the /api/ endpoint
        html! {
            <div>
                {"List of transactions will be displayed here."}
            </div>
        }
    }
}
