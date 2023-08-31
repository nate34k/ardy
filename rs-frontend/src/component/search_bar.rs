use yew::prelude::*;

pub struct SearchBar {
    item_name: String,
}

pub enum Msg {
    UpdateItemName(String),
    Search,
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            item_name: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateItemName(name) => self.item_name = name,
            Msg::Search => {
                // TODO: Implement the search functionality
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    placeholder="Enter item name"
                    value={self.item_name.clone()}
                    //oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateItemName(e.data().unwrap_or("null".to_string())))}
                />
                <button onclick={ctx.link().callback(|_| Msg::Search)}>
                    {"Search"}
                </button>
            </div>
        }
    }
}
