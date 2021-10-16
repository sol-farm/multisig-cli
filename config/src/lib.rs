use anyhow::Result;
use serde::{Deserialize, Serialize};
use simplelog::*;
use anchor_client::{Client, Cluster};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, read_keypair_file};
use std::{fs::File};
use std::{fs};
use rand::rngs::OsRng;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
/// main configuration object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub key_path: String,
    pub db_url: String,
    pub log_file: String,
    pub debug_log: bool,
    pub rpc_url: String,
    pub ws_url: String,
    pub multisig: MultiSignature,
}

/// multisignature related configurations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiSignature {
    pub program_id: String,
    pub accounts: Vec<MultiSigAccount>,
}

/// an instance of the multisignature account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiSigAccount {
    pub name: String,
    pub account: String,
    pub pda: String,
    pub pda_nonce: u8,
    pub threshold: u64,
    pub owners: Vec<String>,
}

impl MultiSigAccount {
    pub fn account(&self) -> Pubkey {
        Pubkey::from_str(self.account.as_str()).unwrap()
    }
    pub fn pda(&self) -> Pubkey {
        Pubkey::from_str(self.pda.as_str()).unwrap()
    }
}

impl MultiSignature {
    /// returns the program id of the deployed multisig program
    pub fn program_id(&self) -> Pubkey {
        Pubkey::from_str(self.program_id.as_str()).unwrap()
    }
    /// returns the multisig struct by searching for its name
    pub fn by_name(&self, name: String) -> Option<MultiSigAccount> {
        for account in self.accounts.iter() {
            if account.name.eq(&name) {
                return Some(account.clone())
            }
        }
        None
    }
    /// returns the index of the multisig account
    pub fn multisig_index(&self, name: String) -> Option<usize> {
        for (idx, account) in self.accounts.iter().enumerate() {
            if account.name.eq(&name) {
                return Some(idx)
            }
        }
        None
    }
}


impl Configuration {
    pub fn new(path: &str, as_json: bool) -> Result<()> {
        let config = Configuration::default();
        config.save(path, as_json)
    }
    pub fn save(&self, path: &str, as_json: bool) -> Result<()> {
        let data = if as_json {
            serde_json::to_string_pretty(&self)?
        } else {
            serde_yaml::to_string(&self)?
        };
        fs::write(path, data).expect("failed to write to file");
        Ok(())
    }
    pub fn load(path: &str, from_json: bool) -> Result<Configuration> {
        let data = fs::read(path).expect("failed to read file");
        let config: Configuration = if from_json {
            serde_json::from_slice(data.as_slice())?
        } else {
            serde_yaml::from_slice(data.as_slice())?
        };
        Ok(config)
    }
    pub fn rpc_client(&self) -> RpcClient {
        RpcClient::new(self.rpc_url.to_string())
    }
    // returns the primary rpc provider
    pub fn get_client(&self) -> Client {
        // just generate a random keypair
        let keypair = Keypair::generate(&mut OsRng);
        let cluster = Cluster::Custom(
            self.rpc_url.clone(),
            self.ws_url.clone(),
        );
        Client::new_with_options(cluster, keypair, CommitmentConfig::confirmed())
    }
    pub fn payer(&self) -> Keypair {
        read_keypair_file(self.key_path.clone()).expect("failed to read keypair file")
    }
    /// if file_log is true, log to both file and stdout
    /// otherwise just log to stdout
    pub fn init_log(&self, file_log: bool) -> Result<()> {
        if !file_log {
            if self.debug_log {
                TermLogger::init(
                    LevelFilter::Debug,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Debug)
                        .build(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                )?;
                return Ok(());
            } else {
                TermLogger::init(
                    LevelFilter::Info,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Error)
                        .build(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                )?;
                return Ok(());
            }
        }
        if self.debug_log {
            CombinedLogger::init(vec![
                TermLogger::new(
                    LevelFilter::Debug,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Debug)
                        .build(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Debug,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Debug)
                        .build(),
                    File::create(self.log_file.as_str()).unwrap(),
                ),
            ])?;
        } else {
            CombinedLogger::init(vec![
                TermLogger::new(
                    LevelFilter::Info,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Error)
                        .build(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Info,
                    ConfigBuilder::new()
                        .set_location_level(LevelFilter::Error)
                        .build(),
                    File::create(self.log_file.as_str()).unwrap(),
                ),
            ])?;
        }

        Ok(())
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            key_path: "~/.config/solana/id.json".to_string(),
            db_url: "postgres://postgres:necc@postgres/kek".to_string(),
            ws_url: "ws://solana-api.projectserum.com".to_string(),
            log_file: "template.log".to_string(),
            debug_log: false,
            rpc_url: "https://solana-api.projectserum.com".to_string(),
            multisig: MultiSignature{
                program_id: "msigmtwzgXJHj2ext4XJjCDmpbcMuufFb5cHuwg6Xdt".to_string(),
                accounts: vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
