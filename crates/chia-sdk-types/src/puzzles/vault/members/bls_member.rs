use chia_bls::PublicKey;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct BlsMember {
    pub public_key: PublicKey,
}

pub const BLS_MEMBER: [u8; 41] = hex!(
    "
    ff02ffff01ff04ffff04ff02ffff04ff05ffff04ff0bff80808080ff8080ffff
    04ffff0132ff018080
    "
);

pub const BLS_MEMBER_HASH: TreeHash = TreeHash::new(hex!(
    "21a3ae8b3ce64d41ca98d6d8df8f465c9e1bfb19ab40284a5da8479ba7fade78"
));
