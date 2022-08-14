use crate::websocket::interface::IWebSocket;
use json::{JsonValue};

pub struct WebSocket<'a> {
    socket: &'a mut dyn IWebSocket
}

impl <'a> WebSocket<'a> {

    pub fn new(socket: &'a mut dyn IWebSocket) -> Self {
        Self { socket }
    }

    pub fn connect(&mut self) {
        self.socket.connect();
    }

    pub fn send(&mut self, msg:JsonValue) {
        self.socket.send(msg);
    }

    pub fn receive(&mut self) -> Result<JsonValue, String> {
        return self.socket.receive();
    }

}