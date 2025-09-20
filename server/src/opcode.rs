#[derive(Debug, Clone, Copy, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Opcode {
    Dispatch = 0,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    ServerStatusUpdate,
    PlayerJoined,
    PlayerLeft,
}
