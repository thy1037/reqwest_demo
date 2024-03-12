use reqwest::RequestBuilder;

use super::{invoke::Invoke, outcome::OutCome};

#[derive(Copy, Clone)]
pub struct Baidu {
    name: &'static str,
}

impl Baidu {
    pub fn new(name: &'static str) -> Baidu {
        Baidu {
            name
        }
    }
}

impl Invoke for Baidu {
    fn request_builder(&self, msg: &str) -> RequestBuilder {
        reqwest::Client::new()
        .post("https://www.baidu.com")
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
