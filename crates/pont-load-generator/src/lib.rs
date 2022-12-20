use anyhow::Result;
use log::{error, info, log_enabled, Level};
use std::collections::HashMap;

pub mod actions;

use crate::actions::{
    create_account_on_chain, create_local_account, fast_transaction, get_shared_resource,
    publish_module,
};
use actions::long_transaction;
use futures::future::join_all;
use pont_sdk::types::LocalAccount;

use csv::{Reader, Writer};
use pont_crypto::ValidCryptoMaterialStringExt;
use std::path::Path;

use framework::{BuildOptions, BuiltPackage};

pub struct Generator {
    pub accounts: Vec<LocalAccount>,
    pub amount: usize,
}

impl Generator {
    pub async fn new(found_account: Option<&mut LocalAccount>, amount: usize) -> Self {
        let mut accounts = vec![];
        let mut m: HashMap<String, String> = HashMap::new();
        let csv_name = "accounts_2000.csv";

        if !Path::new(csv_name).exists() {
            info!("Creating {csv_name}");
            let mut wtr = Writer::from_path(csv_name).unwrap();

            for _ in 0..amount {
                let new_account = LocalAccount::generate(&mut rand::rngs::OsRng);
                accounts.push(new_account);
            }

            let found_account = match found_account {
                Some(account) => account,
                None => {
                    error!("no account provided");
                    return Generator {
                        accounts: vec![],
                        amount: 0,
                    };
                }
            };

            // TODO: make concurrency here
            let mut count = 0;
            for new_account in accounts.iter_mut() {
                count += 1;
                create_account_on_chain(found_account, new_account)
                    .await
                    .unwrap();
                info!("#1 {count}");
            }

            count = 0;
            for new_account in accounts.iter_mut() {
                count += 1;
                fast_transaction(found_account, new_account.address(), 10000000000)
                    .await
                    .unwrap();
                info!("#2 {count}");
            }

            info!("all accounts created");

            for new_account in accounts.iter() {
                wtr.write_record(&[
                    new_account.address().to_hex_literal(),
                    new_account.private_key().to_encoded_string().unwrap(),
                ])
                .unwrap();
            }

            wtr.flush().unwrap();
        } else {
            if log_enabled!(Level::Info) {
                info!("found csv");
                info!("creating local accounts");
            }

            let mut rdr = Reader::from_path("accounts_2000.csv").unwrap();
            let mut tasks = vec![];

            for (i, result) in rdr.records().enumerate() {
                let record = result.unwrap();
                let address = record.get(0).unwrap().to_owned();
                let private_key = record.get(1).unwrap().to_owned();

                m.insert(address, private_key);
                if i > amount {
                    break;
                }
            }

            for (addr, key) in &m {
                tasks.push(create_local_account(addr, &key[2..]));
            }
            accounts = join_all(tasks)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
        }
        Generator { accounts, amount }
    }

    pub async fn start_fast_transactions(&mut self) -> Result<()> {
        info!("fast transaction started");
        let mut tasks = vec![];
        let main_address = self.accounts[0].address();

        // we send coin to self.accounts[0]
        for account in &mut self.accounts[1..self.amount] {
            tasks.push(fast_transaction(account, main_address, 1));
        }

        join_all(tasks).await;

        Ok(())
    }

    pub async fn fund_all_accounts(&mut self, main_address: &mut LocalAccount) -> Result<()> {
        info!("founding started");
        for account in &mut self.accounts[..self.amount] {
            fast_transaction(main_address, account.address(), 10000000000).await?
        }

        Ok(())
    }

    pub async fn start_long_transactions(&mut self) -> Result<()> {
        info!("long transaction started");
        let mut tasks = vec![];
        for account in &mut self.accounts[..self.amount] {
            tasks.push(long_transaction(account));
        }

        for el in join_all(tasks).await {
            el?;
        }

        Ok(())
    }

    pub async fn start_module_publishing(&mut self) -> Result<()> {
        info!("module publishing started");
        let mut tasks = vec![];

        let build_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("mv")
            .canonicalize()?;
        let package = BuiltPackage::build(build_path, BuildOptions::default())?;

        info!("module built");

        for account in &mut self.accounts[..self.amount] {
            tasks.push(publish_module(account, package.clone()));
        }

        join_all(tasks).await;

        Ok(())
    }

    pub async fn start_resource_transactions(&mut self) -> Result<()> {
        info!("start shared resource test");
        let mut tasks = vec![];

        for account in &mut self.accounts[..self.amount] {
            tasks.push(get_shared_resource(account));
        }

        join_all(tasks).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use log::{info, log_enabled, Level};
    use std::collections::HashMap;
    use std::time::Instant;

    use crate::actions::long_transaction;
    use crate::actions::{
        create_account_on_chain, create_local_account, fast_transaction, get_shared_resource,
        publish_module, NODE_URL, PONT_CHAIN_ID,
    };
    use futures::future::join_all;
    use pont_sdk::types::LocalAccount;

    use csv::{Reader, Writer};
    use pont_crypto::ValidCryptoMaterialStringExt;
    use std::path::Path;

    use crate::Generator;
    use framework::{BuildOptions, BuiltPackage};

    pub fn pont_info() {
        match env_logger::try_init() {
            Ok(()) => {}
            Err(_) => {}
        };

        if !log_enabled!(Level::Info) {
            info!("{:?}", NODE_URL.clone());
            info!("Pontem version: {:?}", PONT_CHAIN_ID);
        }
    }

    #[tokio::test]
    async fn test_fast_transactions() {
        pont_info();

        let amount = 1998;
        let mut generator = Generator::new(None, amount).await;
        let now = Instant::now();

        generator
            .start_fast_transactions()
            .await
            .unwrap();
        if log_enabled!(Level::Info) {
            info!("\nFast transactions (transfer):");
            info!(
                "for {} transactions : {} seconds",
                amount,
                now.elapsed().as_secs()
            );
        }
    }

    #[tokio::test]
    async fn test_long_transactions() {
        pont_info();

        let amount = 1998;
        let mut generator = Generator::new(None, amount).await;

        let now = Instant::now();
        generator.start_long_transactions().await.unwrap();
        if log_enabled!(Level::Info) {
            info!("\nLong transactions:");
            info!(
                "for {} transactions : {} seconds",
                amount,
                now.elapsed().as_secs()
            );
        }
    }

    #[tokio::test]
    async fn test_module_publishing() {
        pont_info();

        let amount = 1998;
        let mut generator = Generator::new(None, amount).await;

        let now = Instant::now();
        generator.start_module_publishing().await.unwrap();
        if log_enabled!(Level::Info) {
            info!("\nModule publishing:");
            info!(
                "for {} transactions : {} seconds",
                amount,
                now.elapsed().as_secs()
            );
        }
    }

    #[tokio::test]
    async fn test_shared_resource() {
        pont_info();

        let amount = 1998;
        let mut generator = Generator::new(None, amount).await;

        let now = Instant::now();
        generator.start_resource_transactions().await.unwrap();
        if log_enabled!(Level::Info) {
            info!("\nGet shared resource (un_mut):");
            info!(
                "for {} transactions : {} seconds",
                amount,
                now.elapsed().as_secs()
            );
        }
    }
}
