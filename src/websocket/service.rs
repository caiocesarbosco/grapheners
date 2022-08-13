use crate::websocket::interface::IWebSocket;
use json::{JsonValue};

pub struct WebSocket<'a> {
    socket: &'a mut dyn IWebSocket,
    url: String
}

impl <'a> WebSocket<'a> {

    pub fn new(socket: &'a mut dyn IWebSocket, url: String) -> Self {
        Self { socket, url }
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