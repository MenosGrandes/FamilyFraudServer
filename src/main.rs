use msg_handler::messages::Message;
use msg_handler::msg_handler::MessagaHandler;
use msg_handler::msg_handler_impl::msg_handler_json::MsgHandlerJson;
use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use tungstenite::accept;
mod msg_handler;
/// A WebSocket echo server
/// /*
///https://github.com/snapview/tokio-tungstenite/blob/master/examples/server.rs
/// */
use futures_channel::mpsc::{unbounded, UnboundedSender};
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
fn main() {
    let server: TcpListener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("WS STARTED! {:?}", websocket);
            let msg_handler = MsgHandlerJson::new();

            loop {
                let msg = websocket.read().unwrap();
                let msg_to_send = match msg_handler.handle(msg)
                {
                    Ok(m) =>m,
                    Err(_) => {panic!("aaa")},
                };

                println!("Msg To send : {:?}", msg_to_send);
                /*
                let match msg_to_send.reply_dir {
                    msg_handler::msg_handler_impl::msg_handler_json::ReplyDir::NoReply => todo!(),
                    msg_handler::msg_handler_impl::msg_handler_json::ReplyDir::Admin => todo!(),
                    msg_handler::msg_handler_impl::msg_handler_json::ReplyDir::Client => todo!(),
                    msg_handler::msg_handler_impl::msg_handler_json::ReplyDir::Both => todo!(),
                }*/
                
                if msg_to_send.msg.is_binary() || msg_to_send.msg.is_text() {
                    websocket.send(msg_to_send.msg).unwrap();
                } 
            }
        });
    }
}
