mod bootloader;
mod errors;
mod helper_sync_and_load_programmer;
mod memory_info;
mod message;
mod packet;
mod read_flash;
mod sync;
mod test_port;
mod write_flash;

pub const BES_PROGRAMMING_BAUDRATE: u32 = 921600;
pub const BES_SYNC: u8 = 0xBE;
pub const FLASH_BUFFER_SIZE: usize = 0x8000;

pub use bootloader::load_programmer_runtime_binary_blob;
pub use bootloader::start_programmer_runtime_binary_blob;
pub use errors::BESLinkError;
pub use helper_sync_and_load_programmer::helper_sync_and_load_programmer;
pub use memory_info::query_memory_info;
pub use message::BesMessage;
pub use message::MessageTypes;
pub use packet::read_packet;
pub use packet::send_packet;
pub use read_flash::read_flash_data;
pub use sync::sync;
pub use write_flash::burn_image_to_flash;