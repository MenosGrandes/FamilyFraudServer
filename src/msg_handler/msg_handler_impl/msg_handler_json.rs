use crate::msg_handler::{
    decoder::decoders::Decoder, messages::{Message, self}, msg_handler::MessagaHandler,
    encoder::Encoder
};

pub struct MsgHandlerJson {}
impl MsgHandlerJson {
    pub(crate) fn new() -> Self {
        MsgHandlerJson {}
    }
    
}
pub enum Error {}
#[derive(Debug)]
pub enum ReplyDir
{
    NoReply,
    Admin,
    Client,
    Both,
}
#[derive(Debug)]
pub struct ReturnT
{
    pub msg : tungstenite::Message,
    pub reply_dir : ReplyDir,
}
impl MessagaHandler<tungstenite::Message,messages::Message, Error, ReturnT> for MsgHandlerJson {
    fn handle(&self, msg: tungstenite::Message) -> Result<ReturnT, Error> {
        let m = Message::decode(&msg);
        println!("{:?}", m);

        let r = match self.reply(&m)
        {
            Ok(r) => r,
            Err(_) => panic!("a"),
        };
        Ok(r)
    }

    fn reply(&self, msg : &messages::Message)-> Result<ReturnT, Error> {
        let rd = match msg{
            Message::ShowAnswer(v) => {todo!()},
            Message::GetNewAnswers(v) => {ReturnT{msg : v.encode(), reply_dir : ReplyDir::Both}},
            Message::LifeLost(v) => todo!(),
            Message::SetId(v) => {ReturnT{msg : tungstenite::Message::text(""), reply_dir : ReplyDir::NoReply}},
            Message::Unknown => todo!(),
        };
        Ok(rd)
    }
}
//pub trait MessagaHandler<MsgT, ErrorT>{
