// Generated by Binserde 0.1.0

use bincode::{error, Decode, Encode};
use bincode_macro::Serde;

#[derive(Serde, Encode, Decode, PartialEq, Clone, Debug, Default)]
pub struct ArrayTest {
    pub a: [u8; 32],
    pub b: [Test5; 2],
}

#[derive(Serde, Encode, Decode, PartialEq, Clone, Debug, Default)]
pub struct Test5 {
    pub a: u16,
}
