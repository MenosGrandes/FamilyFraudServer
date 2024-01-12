use serde_json::Value;

use super::topics::Topic;

struct MessageBase {
    topic: Topic,
}
impl MessageBase {
    fn new(topic: Topic) -> Self {
        MessageBase { topic }
    }
}
#[derive(Debug, PartialEq)]
pub enum Message {
    ShowAnswer(u8),
    GetNewAnswers(u8),
    LifeLost,
    SetId(u8),
}
impl TryFrom<Value> for Message {
    type Error = ();
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let topic =
            Topic::try_from(v["topic"].as_u64().unwrap_or_default() as u8).unwrap_or_default();

        match topic {
            x if x == Topic::ShowAnswer => {
                let index = v["index"].as_u64().unwrap_or_default() as u8;
                return Ok(Message::ShowAnswer(index));
            }
            x if x == Topic::GetNewAnswers => {
                let amount_of_answers = v["amount_of_answers"].as_u64().unwrap_or_default() as u8;
                println!("{}", v);
                return Ok(Message::GetNewAnswers(amount_of_answers));
            }
            x if x == Topic::LifeLost => {
                return Ok(Message::LifeLost);
            }
            x if x == Topic::SetId => {
                let type_ = v["type"].as_u64().unwrap_or_default() as u8;
                return Ok(Message::SetId(type_));
            }
            _ => Err(()),
        }
    }
}
/*
pub struct ShowAnswer
{
    base : MessageBase,
    index : u8
}
pub struct GetNewAnswers
{
    base : MessageBase,
    amount_of_answers : u8
}
pub struct LifeLost
{
    base : MessageBase,
}
pub struct SetId
{
    base : MessageBase,
    _type : u8
}
 */