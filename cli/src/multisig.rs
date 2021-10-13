use std::mem;
use config::{Configuration, MultiSigAccount};
use anyhow::Result;
use serum_multisig::Multisig;
use solana_remote_wallet::remote_wallet;
use solana_clap_utils::keypair::DefaultSigner;
use std::str::FromStr;
use rand::rngs::OsRng;
use anchor_client::{RequestNamespace, solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction, sysvar,
}};
use solana_clap_utils::keypair::signer_from_path;
use crate::request_builder;
pub fn new_multisig_config(matches: &clap::ArgMatches, config_file_path: String) -> Result<()> {
    let mut config = Configuration::load(config_file_path.as_str(), false)?;
    let owners: String = matches.values_of("owners").unwrap().collect();
    let owners: Vec<String> = owners.split(",").map(|x| x.to_string()).collect();
    let threshold = matches.value_of("threshold").unwrap();
    config.multisig.accounts.push(MultiSigAccount { 
        name: matches.value_of("name").unwrap().to_string(),
        account: "".to_string(), 
        threshold: u64::from_str(threshold).unwrap(),
        owners,
    });
    config.save(&config_file_path, false)?;
    Ok(())
}
pub fn create_multisig(matches: &clap::ArgMatches, config_file_path: String) -> Result<()> {
    let mut config = Configuration::load(config_file_path.as_str(), false)?;
    let multisig_idx = config.multisig.multisig_index(matches.value_of("name").unwrap().to_string()).unwrap();
    let multisig_config = config.multisig.by_name(matches.value_of("name").unwrap().to_string()).unwrap();
    let multisig_account = Keypair::generate(&mut OsRng);
    config.multisig.accounts[multisig_idx].account = multisig_account.pubkey().to_string();
    let mut wallet_manager = remote_wallet::maybe_wallet_manager().unwrap();
    let signer = signer_from_path(matches, matches.value_of("keypair").unwrap(), matches.value_of("keypair").unwrap(), &mut wallet_manager);
    if signer.is_err() {
        panic!("failed to get signer {:#?}", signer.err().unwrap());
    }
    let signer = signer.unwrap();
    let client = config.get_client();
    let program = client.program(config.multisig.program_id());
    // these are for the program itself
    let (
        multisig_signer,
        multisig_nonce
    ) = {
        Pubkey::find_program_address(
            &[multisig_account.pubkey().as_ref()],
            &config.multisig.program_id(),
        )
    };

    let builder = request_builder::RequestBuilder::from(
        config.multisig.program_id(),
        config.rpc_url.as_str(),
        &*signer,
        None,
        RequestNamespace::Global,
    );
    let sig = builder
    .instruction(system_instruction::create_account(
        &signer.pubkey(),
        &multisig_account.pubkey(),
        program.rpc().get_minimum_balance_for_rent_exemption(500).unwrap(),
        500 as u64,
        &config.multisig.program_id(),
    ))
    .args(serum_multisig::instruction::CreateMultisig{
        owners: multisig_config.owners.iter().map(|owner| Pubkey::from_str(owner).unwrap()).collect(),
        threshold: multisig_config.threshold,
        nonce: multisig_nonce,
    })
    .accounts(serum_multisig::accounts::CreateMultisig{
        multisig: multisig_account.pubkey(),
        rent: sysvar::rent::ID,
    })
    .signer(&*signer)
    .signer(&multisig_account)
    .send(true);
    if sig.is_err() {
        panic!("failed to send tx {:#?}", sig.err().unwrap());
    }
    println!("sent tx {}", sig.unwrap());
    config.save(config_file_path.as_str(),false)?;
    Ok(())
}