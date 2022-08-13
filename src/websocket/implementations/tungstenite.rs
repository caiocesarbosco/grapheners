use std::net::TcpStream;
use tungstenite::{connect, Message, WebSocket, stream::MaybeTlsStream };
use url::Url;
use crate::websocket::interface::IWebSocket;
use json::{JsonValue, object, stringify};

pub struct Tungstenite {
    socket: WebSocket<MaybeTlsStream<TcpStream>>
}

impl Tungstenite {
    pub fn new() -> Self {
        let (socket, _response) = connect(Url::parse("wss://127.0.0.1:8090").unwrap()).expect("Can't connect");
        Self { socket }
    }
}

impl IWebSocket for Tungstenite {

    fn connect(&mut self) {

    }

    fn send(&mut self, msg:JsonValue) {
        self.socket.write_message(Message::Text(stringify(msg)));
    }

    fn receive(&mut self) -> Result<JsonValue, String> {
        let msg = self.socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
        let req = object!{
            msg: ""
        };
        return Ok(req);
    }
}
