use serde_json::{Value, Error};
use tungstenite::Message;

use crate::msg_handler;

pub struct MessagaHandler{}

impl MessagaHandler
{
    pub fn new() -> Self
    {
        MessagaHandler {}
    }
    pub fn handle(&self,msg : Message ) -> Result<(), Error>
    {
        println!("HANDLING : {}", msg);
        let json_data = msg.to_text().unwrap_or_default();

        let v: Value = match serde_json::from_str(json_data)
            {
                Ok(v) => v,
                Err(_) => panic!("ERROR IN MSG"),
            };
            let msg = msg_handler::messages::Message::try_from(v).unwrap();
            println!("{:?}", msg);
            /*
            let topic = Topic::try_from(v["topic"].as_u64().unwrap_or_default() as u8);
            println!("{}",topic.unwrap());
 */

            Ok(())
    }
}