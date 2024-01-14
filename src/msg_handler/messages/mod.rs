use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShowAnswer {
    pub index: u8,
}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetNewAnswers {
    pub amount_of_answers: u8,
}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LifeLost {}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetId {
    pub _type: u8,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    ShowAnswer(ShowAnswer),
    GetNewAnswers(GetNewAnswers),
    LifeLost(LifeLost),
    SetId(SetId),
    Unknown,
}
impl Default for Message {
    fn default() -> Self {
        Message::Unknown
    }
}
