use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{console};
use yewtil::store::{StoreWrapper, Bridgeable, ReadOnly};

use crate::agents::camera_manager::{CameraManager, Request};

pub struct App {
    link: ComponentLink<Self>,
    media_manager: Box<dyn Bridge<StoreWrapper<CameraManager>>>
}

pub enum Msg {
    GetStream,
    GetDevices,
    CameraManagerMsg(ReadOnly<CameraManager>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::CameraManagerMsg);
        let media_manager = CameraManager::bridge(callback);
        Self {
            link,
            media_manager
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStream => {
                self.media_manager.send(Request::GetStream);
                console::log_1(&"after send".into());
            },
            Msg::GetDevices => self.media_manager.send(Request::GetDevices),
            Msg::CameraManagerMsg(state) => {
                if let Some(stream) = &state.borrow().media_stream {
                    console::log_2(&"We have a stream".into(), &stream);
                }

                if let Some(constraints) = &state.borrow().supported_constraints {
                    let as_str = constraints
                        .iter()
                        .map(|constr| constr.to_string() + ", ")
                        .collect::<String>();
                    console::log_2(&"We have constraints:".into(), &as_str.into());
                }
            },
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
