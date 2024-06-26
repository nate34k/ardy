use gloo::console::log;
use reqwasm::http::Request;
use yew::prelude::*;

pub struct ProfitLoss {
    state: State,
}

pub struct State {
    component_ready: bool,
    profit_loss: i64,
}

pub enum Msg {
    GetProfitLossComplete(i64),
    UpdateProfitLoss,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub update_counter: i64,
}

impl Component for ProfitLoss {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        log!(format!("ctx.props().update_counter: {}", ctx.props().update_counter));

        ctx.link().send_future(async {
            let resp = Request::get("http://localhost:43211/api/v1/profit_loss")
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

    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        log!(format!("props.update: {}", props.update_counter));
        ctx.link().send_message(Msg::UpdateProfitLoss);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProfitLossComplete(profit_loss) => {
                self.state.profit_loss = profit_loss;
                self.state.component_ready = true;
                true
            },
            Msg::UpdateProfitLoss => {
                log!("Waiting 50ms before updating profit/loss");
                log!("Updating profit/loss");
                self.state.component_ready = false;

                ctx.link().send_future(async {
                    let resp = Request::get("http://localhost:43211/api/v1/profit_loss")
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
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Helper function to format the number with commas.
        fn format_with_commas(n: i64) -> String {
            let is_negative = n < 0;
            let mut s = n.abs().to_string();
            let mut pos = s.len() as isize - 3;
            
            while pos > 0 {
                s.insert(pos as usize, ',');
                pos -= 3;
            }
            
            if is_negative {
                s = format!("({})", s);
            }
            
            s
        }
        
        // Conditionally render the profit/loss amount based on whether it's ready or not.
        let formatted_profit_loss = {
            // Assuming self.state.profit_loss is i64 or similar type.
            let color = if self.state.profit_loss >= 0 { "#a6da95" } else { "#ed8796" };
            
            // Using the helper function to format the number with commas.
            let formatted_number = format_with_commas(self.state.profit_loss);
            
            let formatted_string = format!("{} gp", formatted_number);
            
            html! {
                <div class="rounded-div" style={format!("background-color: {};", color)}>
                    { formatted_string }
                </div>
            }
        };
        
        // Main component view.
        html! {
            <>
            <div class="profit-loss">{ formatted_profit_loss }</div>
            </>
        }
}

}