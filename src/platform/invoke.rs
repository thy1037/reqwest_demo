use reqwest::RequestBuilder;

use super::outcome::OutCome;

pub trait Invoke: Send + Sync + Copy {
    fn request_builder(&self, msg: &str) -> RequestBuilder;
    fn outcome(&self, response: &str) -> OutCome;
    fn name(&self) -> &str;
}