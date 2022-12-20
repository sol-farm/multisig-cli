pub mod request_builder;
use anyhow::Result;

use crate::request_builder::RequestBuilder;
use anchor_client::anchor_lang;
use anchor_client::anchor_lang::InstructionData;
use anchor_client::anchor_lang::ToAccountMetas;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::bpf_loader_upgradeable;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::commitment_config::CommitmentLevel;
use anchor_client::solana_sdk::instruction::AccountMeta;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::rent;
use anchor_client::solana_sdk::signature::Signature;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::solana_sdk::system_program;
use anchor_client::solana_sdk::sysvar;
use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::Client;
use anchor_client::ClientError;
use anchor_client::Program;
use anchor_client::RequestNamespace;
use serum_multisig::TransactionAccount;
use std::sync::Arc;
impl<'a> RequestBuilder<'a> {
    pub fn propose_blob_ix(
        &self,
        multisig: Pubkey,
        data: &str
    ) -> Result<Pubkey> {
        let ix_data = base64::decode(data)?;
        let ix: Instruction = bincode::deserialize(&ix_data[..])?;
        println!("sending instruction\n{:#?}", ix);
        panic!("error");
        self.propose_solana_instruction(&multisig, ix)
    }
    pub fn propose_transfer_tokens(
        &self,
        multisig: Pubkey,
        pda: Pubkey,
        source: Pubkey,
        target: Pubkey,
        amount: u64,
    ) -> Result<Pubkey> {
        let ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &source,
            &target,
            &pda,
            &vec![&pda],
            amount,
        )?;
        self.propose_solana_instruction(&multisig, ix)
    }
    pub fn propose_solana_instruction(
        &self,
        multisig: &Pubkey,
        instruction: Instruction,
    ) -> Result<Pubkey> {
        let accounts = instruction
            .accounts
            .iter()
            .map(|account_meta| TransactionAccount {
                pubkey: account_meta.pubkey,
                is_signer: false, // multisig-ui does this
                is_writable: account_meta.is_writable,
            })
            .collect::<Vec<TransactionAccount>>();
        self.create_transaction(
            None,
            *multisig,
            instruction.program_id,
            accounts,
            instruction.data,
        )
    }
    pub fn propose_change_auth(
        &self,
        multisig: &Pubkey,
        buffer: &Pubkey,
        new_auth: &Pubkey,
        current_auth: &Pubkey,
    ) -> Result<Pubkey> {
        self.propose_solana_instruction(
            multisig,
            anchor_lang::solana_program::bpf_loader_upgradeable::set_buffer_authority(
                buffer, 
            current_auth, 
            new_auth
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
