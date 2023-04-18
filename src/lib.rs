mod error;
use algonaut::{algod::v2::Algod, kmd::v1::Kmd, transaction::account::Account};
use error::SandboxError;

pub async fn get_unencrypted_default_wallet(
    algod: &Algod,
    kmd: &Kmd,
) -> Result<Account, SandboxError> {
    let list_response = kmd.list_wallets().await?;

    let wallet_id = list_response
        .wallets
        .into_iter()
        .find(|wallet| wallet.name == "unencrypted-default-wallet")
        .unwrap()
        .id;

    let handle = kmd
        .init_wallet_handle(&wallet_id, "")
        .await?
        .wallet_handle_token;

    let keys = kmd.list_keys(handle.as_str()).await?.addresses;

    for addr in keys {
        let info = algod.account_information(&addr.parse().unwrap()).await?;

        if info.status == "Online" {
            let pk = kmd
                .export_key(handle.as_str(), "", &info.address)
                .await?
                .private_key;
            let seed = pk[0..32].try_into().unwrap();

            return Ok(Account::from_seed(seed));
        }
    }

    Err(SandboxError::General("could not find wallet".into()))
}

pub async fn is_sandbox(algod: &Algod) -> Result<bool, SandboxError> {
    let genesis_id = algod.versions().await?.genesis_id;

    Ok(genesis_id == "sandnet-v1")
}
