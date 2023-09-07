use reqwasm::http::Request;
use yew::prelude::*;

pub struct ProfitLoss {
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
            profit_loss: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProfitLossComplete(profit_loss) => {
                self.profit_loss = profit_loss;
                false
            },
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="profit-loss">
                <h1>{ self.profit_loss }</h1>
            </div>
        }
    }
}