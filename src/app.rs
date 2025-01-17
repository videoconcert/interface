use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use web_sys::{console};

use crate::agents::media_manager::{MediaManager, Request, Output as MMOutput};

pub struct App {
    link: ComponentLink<Self>,
    media_manager: Box<dyn Bridge<MediaManager>>
}

pub enum Msg {
    GetStream,
    GetDevices,
    MediaManagerMsg(MMOutput),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::MediaManagerMsg);
        let media_manager = MediaManager::bridge(callback);
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
            Msg::MediaManagerMsg(MMOutput::GetStreamReceived) => {
                console::log_1(&"Get stream received".into());
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
