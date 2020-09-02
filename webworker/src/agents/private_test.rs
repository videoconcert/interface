use yew::agent::{AgentLink, Agent, Private, HandlerId};
use web_sys::{console};
use serde::{Serialize, Deserialize};

pub struct PrivateTest {
  link: AgentLink<PrivateTest>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateInput(pub i32);

impl Agent for PrivateTest {
  type Reach = Private<Self>;
  type Message = i32;
  type Input = PrivateInput;
  type Output = i32;

  fn create(link: AgentLink<Self>) -> Self {
    console::log_1(&"Creating a private test".into());
    PrivateTest {
      link
    }
  }

  fn update(&mut self, _smsg: Self::Message) {

  }

  fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
    console::log_2(&"Got input".into(), &msg.0.into());

    self.link.respond(id, 123);
  }

  fn name_of_resource() -> &'static str {
    "worker.bundle.js"
  }
}