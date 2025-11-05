use base64::Engine;
use std::fmt::{self, Display, Formatter};

/// Maximum number of players in the lottery.
pub const RL_MAX_NUMBER_OF_PLAYERS: usize = 1024;

/// Maximum number of winners in history.
pub const RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY: usize = 1024;

/// Qubic ID (address) representation - 256-bit value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Id {
    pub data: [u8; 32],
}

impl Id {
    /// Creates a zero address
    pub const fn zero() -> Self {
        Self { data: [0u8; 32] }
    }

    pub fn is_zero(&self) -> bool {
        self.data.iter().all(|&b| b == 0)
    }

    pub fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.data)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base64())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EState {
    Selling = 0,
    Locked = 1,
}

impl Default for EState {
    fn default() -> Self {
        EState::Locked
    }
}
