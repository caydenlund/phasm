use std::fmt::Debug;

pub trait Instruction: Debug + Clone {
    fn encode(&self) -> Vec<u8>;

    fn encode_all(instrs: &[Self]) -> Vec<u8> {
        instrs.iter().flat_map(|i| i.encode()).collect()
    }
}
