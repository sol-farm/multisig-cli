use config::{Configuration, MultiSigAccount};
use anyhow::Result;
use spl_token::solana_program::pubkey::Pubkey;
use std::str::FromStr;
pub fn new_multisig_command(matches: &clap::ArgMatches, config_file_path: String) -> Result<()> {
    let mut config = Configuration::load(config_file_path.as_str(), false)?;
    let owners: Vec<_> = matches.values_of("owners").unwrap().collect();
    let threshold = matches.value_of("threshold").unwrap();
    let owners: Vec<String> = owners.iter().map(|o| o.to_string()).collect();
    config.multisig.accounts.push(MultiSigAccount { 
        account: "".to_string(), 
        threshold: u64::from_str(threshold).unwrap(),
        owners,
    });
    config.save(&config_file_path, false)?;
    Ok(())
}