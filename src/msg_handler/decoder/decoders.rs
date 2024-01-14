pub trait Decoder<FromT, ToT> {
    fn decode(from: &FromT) -> ToT;
}
