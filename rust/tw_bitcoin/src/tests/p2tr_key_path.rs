use super::*;
use crate::{
    keypair_from_wif, Recipient, TransactionBuilder, TxInputP2PKH, TxInputP2TRKeyPath,
    TxOutputP2TRKeyPath,
};
use bitcoin::{key::TweakedPublicKey, PubkeyHash, Txid};
use std::str::FromStr;
use tw_encoding::hex;

pub const ALICE_WIF: &str = "cNDFvH3TXCjxgWeVc7vbu4Jw5m2Lu8FkQ69Z2XvFUD9D9rGjofN1";
pub const BOB_WIF: &str = "cNt3XNHiJdJpoX5zt3CXY8ncgrCted8bxmFBzcGeTZbBw6jkByWB";
pub const GENESIS_BLOCK: &str = "253ca41953c2dc36fd627ac4d1bfdfbaeaa09a4eb018a366e39f9187f0f4b520";
pub const GENESIS_TXID: &str = "c50563913e5a838f937c94232f5a8fc74e58b629fae41dfdffcc9a70f833b53a";

#[test]
fn sign_input_p2pkh_output_p2tr_key_path() {
    // This passed the `bitcoin-cli -retest testmempoolaccept` command.
    const EXPECTED_RAW_SIGNED: &str = "02000000013ab533f8709accfffd1de4fa29b6584ec78f5a2f23947c938f835a3e916305c5000000006b48304502210086ab2c2192e2738529d6cd9604d8ee75c5b09b0c2f4066a5c5fa3f87a26c0af602202afc7096aaa992235c43e712146057b5ed6a776d82b9129620bc5a21991c0a5301210351e003fdc48e7f31c9bc94996c91f6c3273b7ef4208a1686021bedf7673bb058ffffffff01c0aff62901000000225120e01cfdd05da8fa1d71f987373f3790d45dea9861acb0525c86656fe50f4397a600000000";

    const COINBASE_AMOUNT: u64 = ONE_BTC * 50;
    const MINER_FEE: u64 = ONE_BTC / 100;
    const SEND_TO_BOB: u64 = COINBASE_AMOUNT - MINER_FEE;

    let alice = keypair_from_wif(ALICE_WIF).unwrap();
    let bob = keypair_from_wif(BOB_WIF).unwrap();

    // # First transaction: Alice spends the P2PKH coinbase input and creates
    // # a P2WPKH output for Bob.

    // Prepare inputs for Alice.
    let input = TxInputP2PKH::builder()
        .txid(Txid::from_str(GENESIS_TXID).unwrap())
        .vout(0)
        .recipient(Recipient::<PubkeyHash>::from_keypair(&alice))
        .satoshis(COINBASE_AMOUNT)
        .build()
        .unwrap();

    // Prepare outputs for Bob.
    let output = TxOutputP2TRKeyPath::builder()
        .recipient(Recipient::<TweakedPublicKey>::from_keypair(&bob))
        .satoshis(SEND_TO_BOB)
        .build()
        .unwrap();

    // Alice signs the transaction.
    let signed_transaction = TransactionBuilder::new()
        .miner_fee(MINER_FEE)
        .add_input(input.into())
        .add_output(output.into())
        .sign_inputs(alice)
        .unwrap()
        .serialize()
        .unwrap();

    let hex = hex::encode(signed_transaction, false);
    assert_eq!(&hex, EXPECTED_RAW_SIGNED);

    // # Second transaction: Bob spends the P2WPKH input and creates
    // # a P2WPKH output for Alice.

    // Transaction was submitted in regtest env via `sendrawtransaction` and
    // mined with `-generate 1`
    const EXPECTED_RAW_SIGNED_SECOND: &str = "02000000000101ac6058397e18c277e98defda1bc38bdf3ab304563d7df7afed0ca5f63220589a0000000000ffffffff01806de72901000000225120a5c027857e359d19f625e52a106b8ac6ca2d6a8728f6cf2107cd7958ee0787c20140ec2d3910d41506b60aaa20520bb72f15e2d2cbd97e3a8e26ee7bad5f4c56b0f2fb0ceaddac33cb2813a33ba017ba6b1d011bab74a0426f12a2bcf47b4ed5bc8600000000";
    const LATEST_TXID: &str = "9a582032f6a50cedaff77d3d5604b33adf8bc31bdaef8de977c2187e395860ac";
    const SEND_TO_ALICE: u64 = SEND_TO_BOB - MINER_FEE;

    let input = TxInputP2TRKeyPath::builder()
        .txid(Txid::from_str(LATEST_TXID).unwrap())
        .vout(0)
        .recipient(Recipient::<TweakedPublicKey>::from_keypair(&bob))
        .satoshis(SEND_TO_BOB)
        .build()
        .unwrap();

    // Prepare outputs for Bob.
    let output = TxOutputP2TRKeyPath::builder()
        .recipient(Recipient::<TweakedPublicKey>::from_keypair(&alice))
        .satoshis(SEND_TO_ALICE)
        .build()
        .unwrap();

    // Alice signs the transaction.
    let signed_transaction = TransactionBuilder::new()
        .miner_fee(MINER_FEE)
        .add_input(input.into())
        .add_output(output.into())
        .sign_inputs(bob)
        .unwrap()
        .serialize()
        .unwrap();

    let hex = hex::encode(signed_transaction, false);
    assert_eq!(hex, EXPECTED_RAW_SIGNED_SECOND);
}
