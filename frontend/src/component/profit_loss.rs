use std::time::Duration;

use reqwasm::http::Request;
use yew::{prelude::*, platform::time::sleep};

pub struct ProfitLoss {
    state: State,
}

pub struct State {
    component_ready: bool,
    profit_loss: i64,
}

pub enum Msg {
    GetProfitLossComplete(i64),
}

impl Component for ProfitLoss {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let resp = Request::get("http://localhost:5000/api/v1/profit_loss")
                .send()
                .await;

            match resp {
                Ok(resp) => {
                    // Get resp body
                    let profit_loss = resp.json::<i64>().await.unwrap();
                    Msg::GetProfitLossComplete(profit_loss)
                },
                Err(_) => {
                    Msg::GetProfitLossComplete(0)
                },
            }
        });

        Self {
            state: State {
                component_ready: false,
                profit_loss: 0,
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProfitLossComplete(profit_loss) => {
                self.state.profit_loss = profit_loss;
                self.state.component_ready = true;
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.state.component_ready {
            html! {
                <div class="profit-loss">
                    <h1>{ self.state.profit_loss }</h1>
                </div>
            }
        } else {
            html! {
                <div class="profit-loss">
                    <h1>{ "Loading..." }</h1>
                </div>
            }
        }
    }
}