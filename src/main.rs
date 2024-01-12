use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
mod msg_handler;
/// A WebSocket echo server
fn main () {
    let server: TcpListener = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let msg_handler = msg_handler::msg_handler::MessagaHandler::new();

            loop {
                let msg = websocket.read().unwrap();
                let _ = msg_handler.handle(msg);
                /*
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                } */
            }
        });
    }
}