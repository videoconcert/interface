use wasm_bindgen::prelude::*;
use web_sys::{MediaDevices, window, MediaStreamConstraints};
use js_sys::{Reflect, Object};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use serde::{Serialize, Deserialize};

use yewtil::store::{Store, StoreWrapper};
use yew::agent::{AgentLink};

use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  GetStream,
  GetDevices,
  GetSupportedConstraints,
}

#[derive(Debug)]
pub enum Action {
  SetStream(JsValue),
  SetStreamError(JsValue),
  SetDevices(Vec<InputDeviceInfo>),
  SetSupportedConstraints(HashSet<MediaConstraint>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputDeviceInfo {
  device_id: DeviceId,
  group_id: String,
  kind: String,
  label: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MediaConstraint {
  AspectRatio,
  AutoGainControl,
  Brightness,
  ChannelCount,
  ColorTemperature,
  Contrast,
  DeviceId,
  EchoCancellation,
  ExposureCompensation,
  ExposureMode,
  ExposureTime,
  FacingMode,
  FocusDistance,
  FocusMode,
  FrameRate,
  GroupId,
  Height,
  Iso,
  Latency,
  NoiseSuppression,
  PointsOfInterest,
  ResizeMode,
  SampleRate,
  SampleSize,
  Saturation,
  Sharpness,
  Torch,
  WhiteBalanceMode,
  Width,
  Zoom,
}

impl fmt::Display for MediaConstraint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct CameraManager {
  pub known_devices: Vec<InputDeviceInfo>,
  pub media_stream: Option<JsValue>,
  pub media_devices: MediaDevices,
  pub set_stream_error: Option<JsValue>,
  pub supported_constraints: Option<HashSet<MediaConstraint>>,
}

impl Store for CameraManager {
  type Action = Action;
  type Input = Request;

  fn new() -> Self {
    let window = window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();

    Self {
      known_devices: Vec::new(),
      media_stream: None,
      media_devices,
      set_stream_error: None,
      supported_constraints: None,
    }
  }

  fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
    match msg {
      Request::GetStream => {
        let video_settings = Object::new();
        Reflect::set(&video_settings, &"width".into(), &640.into()).unwrap();
        Reflect::set(&video_settings, &"height".into(), &480.into()).unwrap();

        let mut media_constraints = MediaStreamConstraints::new();
        media_constraints.audio(&JsValue::TRUE)
                         .video(&video_settings);

        let media_promise = MediaDevices::get_user_media_with_constraints(
            &self.media_devices,
            &media_constraints).unwrap();

        spawn_local(async move {
            match JsFuture::from(media_promise).await {
              Ok(media) => link.send_message(Action::SetStream(media)),
              Err(e) => link.send_message(Action::SetStreamError(e)),
            }
        });
      }

      Request::GetDevices => {
        // Make sure we know what the supported constraints are
        link.send_input(Request::GetSupportedConstraints);

        let devices_promise = MediaDevices::enumerate_devices(&self.media_devices).unwrap();

        spawn_local(async move {
            let devices = JsFuture::from(devices_promise).await
                            .unwrap()
                            .into_serde::<Vec<InputDeviceInfo>>()
                            .unwrap();

            link.send_message(Action::SetDevices(devices));
        });
      }

      Request::GetSupportedConstraints => {
        let supported_constraints = self.media_devices.get_supported_constraints();
        let constraints = Reflect::own_keys(&supported_constraints)
                            .unwrap()
                            .into_serde::<HashSet<MediaConstraint>>()
                            .unwrap();

        link.send_message(Action::SetSupportedConstraints(constraints));
      }
    }
  }

  fn reduce(&mut self, action: Self::Action) {
    match action {
      Action::SetStream(stream) => {
        self.media_stream = Some(stream);
      },
      Action::SetStreamError(error) => {
        self.set_stream_error = Some(error);
      }
      Action::SetDevices(devices) => {
        self.known_devices = devices;
      }
      Action::SetSupportedConstraints(constraints) => {
        self.supported_constraints = Some(constraints);
      }
    }
  }
}