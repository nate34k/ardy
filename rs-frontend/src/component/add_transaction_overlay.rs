use gloo::console::{error, info};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct AddTransactionOverlay {
    state: State,
    trade: Trade,
}

struct State {
    show_overlay: bool,
    is_purchase_radio_button_checked: bool,
    submit_status: SubmitStatus,
}

enum SubmitStatus {
    Success,
    Failure,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    item_name: String,
    quantity: i64,
    total_price: i64,
    is_purchase: bool,
    timestamp: String,
}

pub enum Msg {
    ToggleOverlay,
    SubmitTrade(SubmitEvent),
    SubmitTradeSuccess,
    SubmitTradeFailure,
    UpdateItemName(String),
    UpdateQuantityTraded(i64),
    UpdateTotalTradeValue(i64),
    UpdateIsPurchase(bool),
    UpdateTimestamp(String),
}

impl Component for AddTransactionOverlay {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State {
                show_overlay: false,
                is_purchase_radio_button_checked: true,
                submit_status: SubmitStatus::None,
            },
            trade: Trade {
                item_name: String::new(),
                quantity: 0,
                total_price: 0,
                is_purchase: true,
                timestamp: String::new(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Match Msg enum to update the state of the component based on the desired action
        match msg {
            // Toggle the overlay and return true to re-render the component
            Msg::ToggleOverlay => {
                self.state.show_overlay = !self.state.show_overlay;
                // Log the value of self.show_overlay
                info!(format!("self.show_overlay: {}", &self.state.show_overlay));

                // Set submit_status to None
                self.state.submit_status = SubmitStatus::None;
            
                true
            }
            // Update the trade struct and return false to prevent re-rendering the component
            Msg::UpdateItemName(name) => {
                self.trade.item_name = name;
                info!(format!("self.trade.item_name: {}", &self.trade.item_name));
                false
            }
            Msg::UpdateQuantityTraded(quantity) => {
                self.trade.quantity = quantity;
                info!(format!(
                    "self.trade.quantity_traded: {}",
                    &self.trade.quantity
                ));
                false
            }
            Msg::UpdateTotalTradeValue(value) => {
                self.trade.total_price = value;
                info!(format!(
                    "self.trade.total_trade_value: {}",
                    &self.trade.total_price
                ));
                false
            }
            Msg::UpdateIsPurchase(is_purchase) => {
                self.trade.is_purchase = is_purchase;
                self.state.is_purchase_radio_button_checked = is_purchase;
                info!(format!(
                    "self.trade.is_purchase: {}",
                    &self.trade.is_purchase
                ));
                false
            }
            Msg::UpdateTimestamp(timestamp) => {
                self.trade.timestamp = timestamp;
                info!(format!("self.trade.timestamp: {}", &self.trade.timestamp));
                false
            }
            // Submit the trade struct to the backend and return true to re-render the component
            Msg::SubmitTrade(event) => {
                // Prevent the default behavior of the event (i.e. prevent the form from submitting)
                event.prevent_default();

                self.state.submit_status = SubmitStatus::None;

                // Convert trade struct to JSON
                let trade_json = serde_json::to_string(&self.trade).unwrap();

                // Send the trade to the backend
                ctx.link().send_future(async {
                    info!(format!("Sending trade: {}", trade_json));
                    let resp = Request::post("http://localhost:5000/api/v1/trade")
                        .header("Content-Type", "application/json")
                        .body(trade_json)
                        .send()
                        .await;

                    // Match the response to determine whether the trade was submitted successfully
                    match resp {
                        // If the trade was submitted successfully, log the response body and return Msg::SubmitTradeSuccess
                        Ok(resp) => {
                            info!(format!("Response: {:?}", resp.body()));
                            Msg::SubmitTradeSuccess
                        }
                        // If the trade submission failed, log the error and return Msg::SubmitTradeFailure
                        Err(e) => {
                            error!(format!("Error: {:?}", e.to_string()));
                            Msg::SubmitTradeFailure
                        }
                    }
                });
                true
            }
            Msg::SubmitTradeSuccess => {
                info!("Trade submitted successfully");
                self.state.submit_status = SubmitStatus::Success;
                true
            }
            Msg::SubmitTradeFailure => {
                info!("Trade submission failed");
                self.state.submit_status = SubmitStatus::Failure;
                true
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This is the HTML that will be rendered by the component
        self.construct_base_component_html(&ctx)
    }
}

impl AddTransactionOverlay {
    fn construct_base_component_html(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button id="add-transaction-button" onclick={ctx.link().callback(|_| Msg::ToggleOverlay)}>
                    {"Add Transaction"}
                </button> {
                    self.construct_overlay_html(&ctx)
                }
            </>
        }
    }

    fn construct_overlay_html(&self, ctx: &Context<Self>) -> Html {
        if self.state.show_overlay {
            html! {
                <div class="overlay-container">
                    <div class="overlay"> {
                        self.construct_overlay_header_html(&ctx)
                    } {
                        self.construct_overlay_body_html(&ctx)
                    } 
                    if let SubmitStatus::Success = self.state.submit_status {
                        { self.construct_success_fail_msg_html(&ctx) }
                    }
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn construct_overlay_header_html(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="overlay-header">
                <h2>{"Add Transaction"}</h2>
            </div>
        }
    }

    fn construct_overlay_body_html(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="overlay-body"> {
                self.construct_form_html(&ctx)
            } {
                self.construct_close_button_html(&ctx)
            }
            </div>
        }
    }

    fn construct_success_fail_msg_html(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="alert-card">
                <div class="bar"></div>
                <span class="material-icons">{"check_circle"}</span>
                <div class="content">
                    <h1>{"Success"}</h1>
                    <p>{"Trade submitted successfully"}</p>
                </div>
            </div>
        }
           
    }

    fn construct_form_html(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form id="add-transaction-form" onsubmit={ctx.link().callback(|event| Msg::SubmitTrade(event))}>
                <div class="label-input-container">
                    <input
                        type = "text"
                        id = "item-name"
                        name = "item-name"
                        required = true
                        oninput = {
                            ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateItemName(input.value())
                            })
                        }
                    />
                    <label for="item-name">{"Item Name"}</label>
                </div>

                <div class="label-input-container">
                    <input
                        type="number"
                        id="quantity-traded"
                        name="quantity-traded"
                        required=true
                        pattern="\\d+"
                        oninput={ctx.link().callback(|e: InputEvent| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                match input.value().parse::<i64>() {
                                    Ok(quantity) => Msg::UpdateQuantityTraded(quantity),
                                    Err(_) => Msg::UpdateQuantityTraded(0),
                                }
                            } else {
                                Msg::UpdateQuantityTraded(0)
                            }
                        })}
                    />
                    <label for="quantity-traded">{"Quantity Traded"}</label>
                </div>

                <div class="label-input-container">
                    <input type="number" id="total-trade-value" name="total-trade-value" required=true
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateTotalTradeValue(input.value().parse::<i64>().unwrap())
                        })}
                    />
                    <label for="total-trade-value">{"Total Trade Value"}</label>
                </div>

                <div class="label-input-container">
                    <div class="radio-button-container">
                        <input type="radio" id="purchase" name="sale-or-purchase" value="purchase" checked={self.state.is_purchase_radio_button_checked}
                            onclick={ctx.link().callback(|_| Msg::UpdateIsPurchase(true))}
                        />
                        <label for="purchase">{"Purchase"}</label>
                    </div>
                    <div class="radio-button-container">
                        <input type="radio" id="sale" name="sale-or-purchase" value="sale" checked={!self.state.is_purchase_radio_button_checked}
                            onclick={ctx.link().callback(|_| Msg::UpdateIsPurchase(false))}
                        />
                        <label for="sale">{"Sale"}</label>
                    </div>
                    <label for="sale-or-purchase">{"Sale or Purchase"}</label>
                </div>

                <div class="label-input-container">
                    <input type="datetime-local" id="timestamp" name="timestamp" required=true
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateTimestamp(input.value())
                        })}
                    />
                    <label for="timestamp">{"Timestamp"}</label>
                </div>

                <button type="submit">{"Submit"}</button>

            </form>
        }
    }

    fn construct_close_button_html(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class="close-button" onclick={ctx.link().callback(|_| Msg::ToggleOverlay)}>
                {"Close"}
            </button>
        }
    }
}
