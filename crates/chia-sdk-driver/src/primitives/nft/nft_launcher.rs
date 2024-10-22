use chia_protocol::Bytes32;
use chia_puzzles::{EveProof, Proof};
use clvm_traits::{clvm_quote, FromClvm, ToClvm};
use clvm_utils::ToTreeHash;
use clvmr::{Allocator, NodePtr};
use cni_sdk_types::{Conditions, TransferNft};

use crate::{did_puzzle_assertion, DriverError, Launcher, Spend, SpendContext};

use super::{Nft, NftInfo, NftMint};

impl Launcher {
    pub fn mint_eve_nft<M>(
        self,
        ctx: &mut SpendContext,
        p2_puzzle_hash: Bytes32,
        metadata: M,
        metadata_updater_puzzle_hash: Bytes32,
        royalty_puzzle_hash: Bytes32,
        royalty_ten_thousandths: u16,
    ) -> Result<(Conditions, Nft<M>), DriverError>
    where
        M: ToClvm<Allocator> + FromClvm<Allocator> + ToTreeHash + Clone,
    {
        let launcher_coin = self.coin();

        let nft_info = NftInfo::new(
            launcher_coin.coin_id(),
            metadata,
            metadata_updater_puzzle_hash,
            None,
            royalty_puzzle_hash,
            royalty_ten_thousandths,
            p2_puzzle_hash,
        );

        let inner_puzzle_hash = nft_info.inner_puzzle_hash();
        let (launch_singleton, eve_coin) = self.spend(ctx, inner_puzzle_hash.into(), ())?;

        let proof = Proof::Eve(EveProof {
            parent_parent_coin_info: launcher_coin.parent_coin_info,
            parent_amount: launcher_coin.amount,
        });

        Ok((
            launch_singleton.create_puzzle_announcement(launcher_coin.coin_id().to_vec().into()),
            Nft::new(eve_coin, proof, nft_info),
        ))
    }

    pub fn mint_nft<M>(
        self,
        ctx: &mut SpendContext,
        mint: NftMint<M>,
    ) -> Result<(Conditions, Nft<M>), DriverError>
    where
        M: ToClvm<Allocator> + FromClvm<Allocator> + ToTreeHash + Clone,
    {
        let transfer_condition = mint.owner.map(|owner| {
            TransferNft::new(
                Some(owner.did_id),
                Vec::new(),
                Some(owner.inner_puzzle_hash),
            )
        });

        let conditions = Conditions::new()
            .create_coin(mint.p2_puzzle_hash, 1, vec![mint.p2_puzzle_hash.into()])
            .extend(transfer_condition.clone());

        let inner_puzzle = ctx.alloc(&clvm_quote!(conditions))?;
        let p2_puzzle_hash = ctx.tree_hash(inner_puzzle).into();
        let inner_spend = Spend::new(inner_puzzle, NodePtr::NIL);

        let (mint_eve_nft, eve_nft) = self.mint_eve_nft(
            ctx,
            p2_puzzle_hash,
            mint.metadata,
            mint.metadata_updater_puzzle_hash,
            mint.royalty_puzzle_hash,
            mint.royalty_ten_thousandths,
        )?;

        eve_nft.spend(ctx, inner_spend)?;

        let mut did_conditions = Conditions::new();

        if let Some(transfer_condition) = transfer_condition {
            did_conditions = did_conditions.assert_puzzle_announcement(did_puzzle_assertion(
                eve_nft.coin.puzzle_hash,
                &transfer_condition,
            ));
        }

        let metadata = eve_nft.info.metadata.clone();

        let child = eve_nft.wrapped_child(
            mint.p2_puzzle_hash,
            mint.owner.map(|owner| owner.did_id),
            metadata,
        );

        Ok((mint_eve_nft.extend(did_conditions), child))
    }
}

#[cfg(test)]
mod tests {
    use crate::{DidOwner, IntermediateLauncher, Launcher, StandardLayer};

    use super::*;

    use chia_consensus::spendbundle_conditions::get_conditions_from_spendbundle;
    use chia_protocol::{Coin, SpendBundle};
    use chia_puzzles::{nft::NftMetadata, standard::StandardArgs};
    use cni_sdk_signer::AggSigConstants;
    use cni_sdk_test::{sign_transaction, test_secret_key, Simulator};
    use cni_sdk_types::{announcement_id, TESTNET11_CONSTANTS};

    #[test]
    fn test_nft_mint_cost() -> anyhow::Result<()> {
        let ctx = &mut SpendContext::new();

        let sk = test_secret_key()?;
        let pk = sk.public_key();
        let p2 = StandardLayer::new(pk);

        let puzzle_hash = StandardArgs::curry_tree_hash(pk).into();
        let coin = Coin::new(Bytes32::new([0; 32]), puzzle_hash, 1);

        let (create_did, did) = Launcher::new(coin.coin_id(), 1).create_simple_did(ctx, &p2)?;
        p2.spend(ctx, coin, create_did)?;

        // We don't want to count the DID creation.
        ctx.take();

        let coin = Coin::new(Bytes32::new([1; 32]), puzzle_hash, 1);
        let (mint_nft, _nft) = IntermediateLauncher::new(did.coin.coin_id(), 0, 1)
            .create(ctx)?
            .mint_nft(
                ctx,
                NftMint::new(NftMetadata::default(), puzzle_hash, 300, None),
            )?;

        let _ = did.update(
            ctx,
            &p2,
            mint_nft.create_coin_announcement(b"$".to_vec().into()),
        )?;

        p2.spend(
            ctx,
            coin,
            Conditions::new().assert_coin_announcement(announcement_id(did.coin.coin_id(), "$")),
        )?;

        let coin_spends = ctx.take();
        let signature = sign_transaction(
            &coin_spends,
            &[sk],
            &AggSigConstants::new(TESTNET11_CONSTANTS.agg_sig_me_additional_data),
        )?;
        let spend_bundle = SpendBundle::new(coin_spends, signature);

        let conds = get_conditions_from_spendbundle(
            &mut ctx.allocator,
            &spend_bundle,
            u64::MAX,
            100_000_000,
            &TESTNET11_CONSTANTS,
        )?;

        assert_eq!(conds.cost, 109_517_025);

        Ok(())
    }

    #[test]
    fn test_bulk_mint() -> anyhow::Result<()> {
        let mut sim = Simulator::new();
        let ctx = &mut SpendContext::new();

        let sk = test_secret_key()?;
        let pk = sk.public_key();
        let p2 = StandardLayer::new(pk);

        let puzzle_hash = StandardArgs::curry_tree_hash(pk).into();
        let coin = sim.new_coin(puzzle_hash, 3);

        let (create_did, did) = Launcher::new(coin.coin_id(), 1).create_simple_did(ctx, &p2)?;
        p2.spend(ctx, coin, create_did)?;

        let mint = NftMint::new(
            NftMetadata::default(),
            puzzle_hash,
            300,
            Some(DidOwner::from_did_info(&did.info)),
        );

        let mint_1 = IntermediateLauncher::new(did.coin.coin_id(), 0, 2)
            .create(ctx)?
            .mint_nft(ctx, mint.clone())?
            .0;

        let mint_2 = IntermediateLauncher::new(did.coin.coin_id(), 1, 2)
            .create(ctx)?
            .mint_nft(ctx, mint)?
            .0;

        let _ = did.update(ctx, &p2, mint_1.extend(mint_2))?;

        sim.spend_coins(ctx.take(), &[sk])?;

        Ok(())
    }

    #[test]
    fn test_nonstandard_intermediate_mint() -> anyhow::Result<()> {
        let mut sim = Simulator::new();
        let ctx = &mut SpendContext::new();

        let sk = test_secret_key()?;
        let pk = sk.public_key();
        let p2 = StandardLayer::new(pk);

        let puzzle_hash = StandardArgs::curry_tree_hash(pk).into();
        let coin = sim.new_coin(puzzle_hash, 3);

        let (create_did, did) = Launcher::new(coin.coin_id(), 1).create_simple_did(ctx, &p2)?;
        p2.spend(ctx, coin, create_did)?;

        let intermediate_coin = Coin::new(did.coin.coin_id(), puzzle_hash, 0);

        let (create_launcher, launcher) = Launcher::create_early(intermediate_coin.coin_id(), 1);

        let mint = NftMint::new(
            NftMetadata::default(),
            puzzle_hash,
            300,
            Some(DidOwner::from_did_info(&did.info)),
        );

        let (mint_nft, _nft) = launcher.mint_nft(ctx, mint)?;

        let _ = did.update(ctx, &p2, mint_nft.create_coin(puzzle_hash, 0, Vec::new()))?;
        p2.spend(ctx, intermediate_coin, create_launcher)?;

        sim.spend_coins(ctx.take(), &[sk])?;

        Ok(())
    }

    #[test]
    fn test_nonstandard_intermediate_mint_recreated_did() -> anyhow::Result<()> {
        let mut sim = Simulator::new();
        let ctx = &mut SpendContext::new();

        let sk = test_secret_key()?;
        let pk = sk.public_key();
        let p2 = StandardLayer::new(pk);

        let puzzle_hash = StandardArgs::curry_tree_hash(pk).into();
        let coin = sim.new_coin(puzzle_hash, 3);

        let (create_did, did) = Launcher::new(coin.coin_id(), 1).create_simple_did(ctx, &p2)?;
        p2.spend(ctx, coin, create_did)?;

        let intermediate_coin = Coin::new(did.coin.coin_id(), puzzle_hash, 0);

        let (create_launcher, launcher) = Launcher::create_early(intermediate_coin.coin_id(), 1);
        p2.spend(ctx, intermediate_coin, create_launcher)?;

        let mint = NftMint::new(
            NftMetadata::default(),
            puzzle_hash,
            300,
            Some(DidOwner::from_did_info(&did.info)),
        );

        let (mint_nft, _nft_info) = launcher.mint_nft(ctx, mint)?;

        let did = did.update(
            ctx,
            &p2,
            Conditions::new().create_coin(puzzle_hash, 0, Vec::new()),
        )?;

        let _ = did.update(ctx, &p2, mint_nft)?;

        sim.spend_coins(ctx.take(), &[sk])?;

        Ok(())
    }
}
