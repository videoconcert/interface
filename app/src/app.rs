use yew::prelude::*;
use web_sys::{console};
use yewtil::store::{StoreWrapper, Bridgeable, ReadOnly};
use wasm_bindgen_futures::{spawn_local};

use crate::agents::camera_manager::{CameraManager, Request};
use webworker::agents::private_test::{PrivateTest, PrivateInput};

pub struct App {
    link: ComponentLink<Self>,
    media_manager: Box<dyn Bridge<StoreWrapper<CameraManager>>>,
    priv_test: Box<dyn Bridge<PrivateTest>>,
}

pub enum Msg {
    GetStream,
    GetDevices,
    CameraManagerMsg(ReadOnly<CameraManager>),
    PrivateMsg(i32)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        console::log_1(&"creating a component".into());
        let callback = link.callback(Msg::CameraManagerMsg);
        let media_manager = CameraManager::bridge(callback);

        let priv_cb = link.callback(Msg::PrivateMsg);
        let priv_test = PrivateTest::bridge(priv_cb);
        Self {
            link,
            media_manager,
            priv_test
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStream => {
                self.media_manager.send(Request::GetStream);
                spawn_local(async {
                    console::log_1(&"after send".into());
                });
            },
            Msg::GetDevices => {
                console::log_2(&"get devices".into(), &"Into".into());
                self.priv_test.send(PrivateInput(123));
                self.media_manager.send(Request::GetDevices)
            },
            Msg::CameraManagerMsg(state) => {
                console::log_1(&"Blah blah".into());
                if let Some(stream) = &state.borrow().media_stream {
                    console::log_2(&"We have a stream".into(), &stream);
                }

                if let Some(constraints) = &state.borrow().supported_constraints {
                    let as_str = constraints
                        .iter()
                        .map(|constr| constr.to_string() + ", ")
                        .collect::<String>();
                    //console::log_2(&"We have constraints:".into(), &as_str.into());
                }
            },
            Msg::PrivateMsg(_) => {
                console::log_1(&"We got a private msg".into());
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::GetStream)>
                    { "get stream" }
                </button>

                <button onclick=self.link.callback(|_| Msg::GetDevices)>
                    { "get devices" }
                </button>
            </div>
        }
    }
}
