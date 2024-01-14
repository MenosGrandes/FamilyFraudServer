use tungstenite::Message;

use super::{
    encoder::Encoder,
    messages::{GetNewAnswers, LifeLost},
};

impl Encoder<Message> for LifeLost {
    fn encode(&self) -> Message {
        Message::Text("".into())
    }
}

impl Encoder<Message> for GetNewAnswers {
    fn encode(&self) -> Message {
        let data = match serde_json::to_string(&self) {
            Ok(d) => d,
            Err(_) => todo!(),
        };

        Message::Text(data)
    }
}
