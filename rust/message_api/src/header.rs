//! Message header structure.
use crate::account_id::AccountID;
use crate::data_pointer::DataPointer;

/// The size of a message header in bytes.
pub const MESSAGE_HEADER_SIZE: usize = size_of::<MessageHeader>();

/// A message header.
#[derive(Default, Copy, Clone)]
#[non_exhaustive]
pub struct MessageHeader {
    /// The target account of the message.
    pub account: AccountID, // 8 bytes
    /// The sender account of the message.
    pub sender_account: AccountID, // 8 bytes
    /// The message selector.
    pub message_selector: u64, // 8 bytes
    /// The context key.
    pub context_key: u64, // 8 bytes
    /// The gas limit.
    pub gas_limit: u64, // 8 bytes
    /// The gas consumed.
    pub gas_consumed: u64, // 8 bytes
    /// Input data pointer 1.
    pub in_pointer1: DataPointer, // 16 bytes
    /// Input data pointer 2.
    pub in_pointer2: DataPointer, // 16 bytes
    /// Output data pointer 1.
    pub out_pointer1: DataPointer, // 16 bytes
    /// Output data pointer 2.
    pub out_pointer2: DataPointer, // 16 bytes

    reserved: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_header_size() {
        assert_eq!(MESSAGE_HEADER_SIZE, 128);
    }
}