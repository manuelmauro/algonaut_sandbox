mod error;
use algonaut::{algod::v2::Algod, kmd::v1::Kmd, transaction::account::Account};
use error::SandboxError;

const ALGOD_URL: &str = "http://localhost:4001";
const ALGOD_TOKEN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const KMD_URL: &str = "http://localhost:4002";
const KMD_TOKEN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

pub struct Sandbox {
    pub algod: Algod,
    pub kmd: Kmd,
}

impl Sandbox {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with(algod: &Algod, kmd: &Kmd) -> Self {
        Sandbox {
            algod: (*algod).clone(),
            kmd: (*kmd).clone(),
        }
    }

    pub async fn unencrypted_default_wallet(self) -> Result<Account, SandboxError> {
        let list_response = self.kmd.list_wallets().await?;

        let wallet_id = list_response
            .wallets
            .into_iter()
            .find(|wallet| wallet.name == "unencrypted-default-wallet")
            .unwrap()
            .id;

        let handle = self
            .kmd
            .init_wallet_handle(&wallet_id, "")
            .await?
            .wallet_handle_token;

        let keys = self.kmd.list_keys(handle.as_str()).await?.addresses;

        for addr in keys {
            let info = self
                .algod
                .account_information(&addr.parse().unwrap())
                .await?;

            if info.status == "Online" {
                let pk = self
                    .kmd
                    .export_key(handle.as_str(), "", &info.address)
                    .await?
                    .private_key;
                let seed = pk[0..32].try_into().unwrap();

                return Ok(Account::from_seed(seed));
            }
        }

        Err(SandboxError::General("could not find wallet".into()))
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Sandbox {
            algod: Algod::new(ALGOD_URL, ALGOD_TOKEN).unwrap(),
            kmd: Kmd::new(KMD_URL, KMD_TOKEN).unwrap(),
        }
    }
}

pub async fn is_sandbox(algod: &Algod) -> Result<bool, SandboxError> {
    let genesis_id = algod.versions().await?.genesis_id;

    Ok(genesis_id == "sandnet-v1")
}
