use serde::{Serialize, Deserialize};

use yew::agent::{AgentLink};
use yewtil::store::{Store, StoreWrapper};

pub struct VideoUpstream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  StartUpload,
}

#[derive(Debug)]
pub enum Action {
  SetUpload,
}

impl Store for VideoUpstream {
  type Action = Action;
  type Input = Request;

  fn new() -> Self {
    Self
  }

  fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {

  }

  fn reduce(&mut self, action: Self::Action) {

  }
}