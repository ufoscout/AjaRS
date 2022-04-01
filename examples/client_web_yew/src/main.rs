use ajars::web::{error::Error, AjarsWeb};
use examples_common::ping::{PingRequest, PingResponse, PING};
use std::rc::Rc;
use yew::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Msg {
    PingSend,
    PingSetResponse(Result<PingResponse, Error>),
}

struct Model {
    ajars: Rc<AjarsWeb>,
    ping_response: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // This should be created at application level and shared across all components and services
        let ajars = { Rc::new(AjarsWeb::new("http://127.0.0.1:3000").expect("Should build Ajars")) };

        Self { ajars, ping_response: "".to_owned() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let ajars = self.ajars.clone();
        match msg {
            Msg::PingSend => {
                ctx.link().send_future(async move {
                    // Performs a GET request to /api/ping
                    // The PingRequest and PingResponse types are enforced at compile time
                    let response = ajars
                        .request(&PING)
                        .send(&PingRequest { message: "Call From web_sys in Yew".to_owned() })
                        .await;

                    Msg::PingSetResponse(response)
                });
                false
            }
            Msg::PingSetResponse(response) => {
                self.ping_response = match response {
                    Ok(response) => {
                        format!("Ping backend response: {:?}", response.message)
                    }
                    Err(err) => {
                        format!("Ping call error: {:?}", err)
                    }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::PingSend);
        html! {
            <div>
                <h1> { "Ping the backend" }</h1>
                <button {onclick}>{ "Ping" }</button>
                <p>{ self.ping_response.clone() }</p>
                <br/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
