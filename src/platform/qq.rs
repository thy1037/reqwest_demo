use reqwest::RequestBuilder;

use super::{invoke::Invoke, outcome::OutCome};

#[derive(Copy, Clone)]
pub struct Qq {
    name: &'static str,
}

impl Qq {
    pub fn new(name: &'static str) -> Qq {
        Qq {
            name
        }
    }
}

impl Invoke for Qq {
    fn request_builder(&self, msg: &str) -> RequestBuilder {
        reqwest::Client::new()
        .post("https://www.qq.com")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Accept", "text/event-stream")
        .header("Content-Type", "application/json;charset=UTF-8")
        .body(msg.to_string())
        
    }

    fn outcome(&self, response: &str) -> OutCome {
        OutCome::new(response, "tip")
    }

    fn name(&self) -> &str {
        self.name
    }
}
