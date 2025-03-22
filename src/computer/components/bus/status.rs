#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BusStatus {
    #[default]
    Idle,
    Read,
    WriteByte,
    WriteHalfWord,
    WriteWord,
    WriteDoubleWord,
}
