pub trait MessagaHandler<MsgFrom, MsgTo, ErrorT, ReturnT> {
    fn handle(&self, msg: MsgFrom) -> Result<ReturnT, ErrorT>;
    fn reply(&self,msg : &MsgTo)-> Result<ReturnT, ErrorT>;
}
/*
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
            /*
            let topic = Topic::try_from(v["topic"].as_u64().unwrap_or_default() as u8);
            println!("{}",topic.unwrap());
 */

            Ok(())
    }
}
*/
