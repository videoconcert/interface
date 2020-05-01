use yew::worker::*;
use yew::agent::{AgentLink};
use wasm_bindgen::prelude::*;
use web_sys::{MediaDevices, window, console, MediaStreamConstraints};
use wasm_bindgen_futures::{JsFuture, spawn_local};

use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  GetStream,
  GetDevices,
}

#[derive(Debug)]
pub enum Message {
  SetStream(JsValue),
  SetDevices(Vec<InputDeviceInfo>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceId(pub String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputDeviceInfo {
  device_id: DeviceId,
  group_id: String,
  kind: String,
  label: String
}

pub struct MediaManager {
  known_devices: Vec<InputDeviceInfo>,
  media_stream: Option<JsValue>,

  subscribers: HashSet<HandlerId>,
  media_devices: MediaDevices,
  link: AgentLink<MediaManager>,
}

impl Agent for MediaManager {
  type Reach = Context;
  type Message = Message;
  type Input = Request;
  type Output = ();

  fn create(link: AgentLink<Self>) -> Self {
    let window = window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();

    MediaManager {
      known_devices: Vec::new(),
      media_stream: None,

      subscribers: HashSet::new(),
      media_devices,
      link
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      Message::SetStream(stream) => {
        console::log_2(&"We have stream".into(), &stream);
        self.media_stream = Some(stream);
      },
      Message::SetDevices(devices) => {
        console::log_2(&"We have devices".into(), &JsValue::from_serde(&devices).unwrap());
        self.known_devices = devices;
      }
    }
  }

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::GetStream => {
        let mut media_constraints = MediaStreamConstraints::new();
        media_constraints.audio(&JsValue::TRUE)
                         .video(&JsValue::TRUE);

        let media_promise = MediaDevices::get_user_media_with_constraints(
            &self.media_devices,
            &media_constraints).unwrap();

        let link = self.link.clone();
        let handler = async move {
            let media = JsFuture::from(media_promise).await.unwrap();
            link.callback(|media| Message::SetStream(media)).emit(media);
        };

        spawn_local(handler);
      }
      Request::GetDevices => {
        let devices_promise = MediaDevices::enumerate_devices(&self.media_devices).unwrap();

        let link = self.link.clone();
        let handler = async move {
            let devices = JsFuture::from(devices_promise).await
                            .unwrap()
                            .into_serde::<Vec<InputDeviceInfo>>()
                            .unwrap();

            link.callback(|devices| Message::SetDevices(devices)).emit(devices);
        };

        spawn_local(handler);
      }
    }
  }

  fn connected(&mut self, id: HandlerId) {
    self.subscribers.insert(id);
  }

  fn disconnected(&mut self, id: HandlerId) {
    self.subscribers.remove(&id);
  }

  fn name_of_resource() -> &'static str {
    "media-manager.js"
  }
}