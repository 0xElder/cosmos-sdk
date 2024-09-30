/// Account ID is a unique integer identifier for an account.
/// Every account has one and only one account identifier.
/// This is distinct from an account's "address".
/// An account may actually have multiple addresses in
/// different "address spaces" from the point of view of
/// an external user, but an account always has one unique account ID.
/// The account ID zero is reserved for the "null account" meaning
/// that the account is not valid or does not exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccountID(u64);

impl AccountID {
    /// Creates a new account ID from the given integer.
    pub const fn new(id: u64) -> Self {
        AccountID(id)
    }

    /// Returns the integer value of the account ID.
    pub fn get(&self) -> u64 {
        self.0
    }

    /// Returns true if the account ID is zero.
    /// The account ID zero is reserved for the "null account" meaning
    /// that the account is not valid or does not exist.
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}