use ajars::web::{error::Error, AjarsWeb};
use ajars_common::ping::{PingRequest, PingResponse, PING};
use std::rc::Rc;
use yew::{prelude::*, services::ConsoleService};
use yewtil::future::LinkFuture;

enum Msg {
    PingSend,
    PingSetResponse(Result<PingResponse, Error>),
}

struct Model {
    link: ComponentLink<Self>,
    ajars: Rc<AjarsWeb>,
    ping_response: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // This should be created at application level and shared across all components and services
        let ajars = { Rc::new(AjarsWeb::new("http://127.0.0.1:3000").expect("Should build Ajars")) };

        Self { ajars, link, ping_response: "".to_owned() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::log("update");
        let ajars = self.ajars.clone();
        match msg {
            Msg::PingSend => {
                self.link.send_future(async move {
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ping = self.link.callback(|_| Msg::PingSend);
        html! {
            <div>
                <h1> { "Ping the backend" }</h1>
                <button onclick=ping>{ "Ping" }</button>
                <p>{ self.ping_response.clone() }</p>
                <br/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}