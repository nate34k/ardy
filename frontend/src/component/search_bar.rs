use web_sys::{wasm_bindgen::{closure::Closure, JsCast}, window, HtmlInputElement};
use yew::prelude::*;

pub struct SearchBar {
    item_name: String,
    timeout_id: Option<i32>,
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
            timeout_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Update the item name used for searching, but wait 750ms before searching after no more
            Msg::UpdateItemName(name) => {
                self.item_name = name;

                // Cancel the current timeout
                if let Some(timeout_id) = self.timeout_id {
                    window()
                        .unwrap()
                        .clear_timeout_with_handle(timeout_id);
                }

                // Clone the context and create a callback for the timeout
                let link = ctx.link().clone();
                let callback = Closure::wrap(Box::new(move || {
                    link.send_message(Msg::Search);
                }) as Box<dyn Fn()>);

                // Set a new timeout and store the timeout ID
                let timeout_id = window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        400,
                    )
                    .expect("Failed to set timeout");

                // Store the timeout ID and forget the closure to avoid it being dropped
                self.timeout_id = Some(timeout_id);
                callback.forget();

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
            <div class="input-box">
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