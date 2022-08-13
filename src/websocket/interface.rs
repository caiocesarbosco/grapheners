use json::{JsonValue};

pub trait IWebSocket {
    fn connect(&mut self);
    fn send(&mut self, msg: JsonValue);
    fn receive(&mut self) -> Result<JsonValue, String>;
}