use crate::msg_handler::{
    decoder::decoders::Decoder,
    messages::{GetNewAnswers, LifeLost, Message, SetId, ShowAnswer},
    msg_handler_impl::topics::Topic,
};
use serde_json::Value;

impl TryFrom<Value> for Message {
    type Error = ();
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let topic =
            Topic::try_from(v["topic"].as_u64().unwrap_or_default() as u8).unwrap_or_default();
        match topic {
            x if x == Topic::GetNewAnswers => Ok(Message::GetNewAnswers(GetNewAnswers::decode(&v))),
            x if x == Topic::ShowAnswer => Ok(Message::ShowAnswer(ShowAnswer::default())),
            x if x == Topic::SetId => Ok(Message::SetId(SetId::default())),
            x if x == Topic::LifeLost => Ok(Message::LifeLost(LifeLost::default())),

            _ => Err(()),
        }
    }
}

impl Decoder<tungstenite::Message, Message> for Message {
    fn decode(from: &tungstenite::Message) -> Message {
        let json_data = from.to_text().unwrap_or_default();
        let data: Value = match serde_json::from_str(json_data) {
            Ok(data) => data,
            Err(e) => panic!("{} ERROR ID DECODING", e),
        };

        let msg = Message::try_from(data).unwrap();

        return msg;
    }
}

impl Decoder<Value, GetNewAnswers> for GetNewAnswers {
    fn decode(from: &Value) -> GetNewAnswers {
        let amount_of_answers = from["amount_of_answers"].as_u64().unwrap_or_default() as u8;
        GetNewAnswers { amount_of_answers }
    }
}

impl Decoder<Value, SetId> for  SetId{
    fn decode(from: &Value) ->  SetId{
        let _type= from["type"].as_u64().unwrap_or_default() as u8;
        SetId{ _type}
    }
}
