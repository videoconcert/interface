use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};

use crate::agents::media_manager::{MediaManager, Request};

pub struct App {
    link: ComponentLink<Self>,
    media_manager: Dispatcher<MediaManager>,
}

pub enum Msg {
    GetStream,
    GetDevices,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            media_manager: MediaManager::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStream => self.media_manager.send(Request::GetStream),
            Msg::GetDevices => self.media_manager.send(Request::GetDevices),
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
