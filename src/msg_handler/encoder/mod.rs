pub trait Encoder<To> {
    fn encode(&self) -> To;
}
