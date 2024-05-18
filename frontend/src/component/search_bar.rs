use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct SearchBar {
    item_name: String,
}

pub enum Msg {
    UpdateItemName(String),
    Search,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub on_search: Callback<String>,
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            item_name: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateItemName(name) => {
                self.item_name = name;
                ctx.props().on_search.emit(self.item_name.clone());
                true
            },
            Msg::Search => {
                let item_name = self.item_name.clone();
                ctx.props().on_search.emit(item_name);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="search-bar">
                <input
                    type="text"
                    placeholder="Enter item name"
                    value={self.item_name.clone()}
                    oninput = {
                        ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateItemName(input.value())
                        })
                    }
                />
            </div>
        }
    }
}