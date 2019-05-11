#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum StreamInstruction {
    Close,
}
