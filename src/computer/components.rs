pub mod bus;
pub mod cpu;
pub mod ram;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// Memory Mapped Component
pub enum MMC {
    #[default]
    RAM,
}
