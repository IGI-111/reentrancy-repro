use fuels::prelude::*;

use fuels::{
    macros::abigen,
    programs::contract::{Contract, LoadConfiguration},
    types::{bech32::Bech32Address, transaction::TxPolicies},
};

abigen!(
    Contract(name = "FooContract", abi = "foo/out/debug/foo-abi.json"),
    Contract(name = "BarContract", abi = "bar/out/debug/bar-abi.json"),
);

#[tokio::test]
async fn trigger_foobar() {
    let wallet = launch_provider_and_get_wallet().await.unwrap();

    let foo_id = Contract::load_from("foo/out/debug/foo.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();
    let foo = FooContract::new(foo_id.clone(), wallet.clone());
    let foo_address: Bech32Address =
        Bech32Address::new(foo.contract_id().hrp(), foo.contract_id().hash());

    let bar_id = Contract::load_from("bar/out/debug/bar.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();
    let bar = BarContract::new(bar_id.clone(), wallet);
    let bar_address: Bech32Address =
        Bech32Address::new(bar.contract_id().hrp(), bar.contract_id().hash());

    let res = foo
        .methods()
        .foo(foo_address, bar_address)
        .with_contracts(&[&foo, &bar])
        .call()
        .await
        .unwrap();
    dbg!(res.receipts);
    assert_eq!(res.value, 42);
}
