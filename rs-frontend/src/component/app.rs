use crate::component::*;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <search_bar::SearchBar />
                <transaction_list::TransactionList />
                <add_transaction_overlay::AddTransactionOverlay />
            </main>
        }
    }
}
