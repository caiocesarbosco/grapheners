use json::{object};
use crate::websocket::service::WebSocket;
use crate::websocket::implementations::tungstenite::Tungstenite;

mod websocket;

fn main () {

    let mut concrete_ws = Tungstenite::new(String::from("wss://127.0.0.1:8090"));
    let mut ws_service = WebSocket::new(&mut concrete_ws);

    let req = object!{
        method: "call",
        params: [1, "login", ["",""]],
        id: 1
    };

    ws_service.send(req);
    ws_service.receive();

    let req = object!{
        method: "call",
        params: [1, "database", []],
        id: 2
    };

    ws_service.send(req);
    ws_service.receive();

    let req = object!{
        method: "call",
        params: [2, "get_chain_id", []],
        id: 3
    };

    ws_service.send(req);
    ws_service.receive();

}
