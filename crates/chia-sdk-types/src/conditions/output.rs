use chia_protocol::{Bytes, Bytes32};
use clvm_traits::{apply_constants, FromClvm, ToClvm};

#[derive(ToClvm, FromClvm)]
#[apply_constants]
#[derive(Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct CreateCoin {
    #[clvm(constant = 51)]
    pub opcode: u8,
    pub puzzle_hash: Bytes32,
    pub amount: u64,
    #[clvm(default)]
    pub memos: Vec<Bytes>,
}

impl CreateCoin {
    #[must_use]
    pub const fn new(puzzle_hash: Bytes32, amount: u64) -> Self {
        Self::with_memos(puzzle_hash, amount, Vec::new())
    }

    #[must_use]
    pub const fn with_memos(puzzle_hash: Bytes32, amount: u64, memos: Vec<Bytes>) -> Self {
        Self {
            puzzle_hash,
            amount,
            memos,
        }
    }

    #[must_use]
    pub fn with_hint(puzzle_hash: Bytes32, amount: u64) -> Self {
        Self::with_custom_hint(puzzle_hash, amount, puzzle_hash)
    }

    #[must_use]
    pub fn with_custom_hint(puzzle_hash: Bytes32, amount: u64, hint: Bytes32) -> Self {
        Self::with_memos(puzzle_hash, amount, vec![hint.into()])
    }
}

#[derive(ToClvm, FromClvm)]
#[apply_constants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct ReserveFee {
    #[clvm(constant = 52)]
    pub opcode: u8,
    pub amount: u64,
}
