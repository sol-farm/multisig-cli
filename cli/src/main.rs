#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
use anyhow::{Result, anyhow};
use tokio;
use clap::{App, Arg, SubCommand};
mod config;
mod multisig;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("template-cli")
    .version("0.0.1")
    .author("solfarm")
    .about("template cli for rust projects")
    .arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("sets the config file")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("keypair")
        .short("k")
        .long("keypair")
        .value_name("KEYPAIR")
        .help("specifies the keypair to use for signing transactions")
        .required(false)
    )
    .subcommand(
        SubCommand::with_name("config")
        .about("configuration management commands")
        .subcommands(
            vec![
                SubCommand::with_name("new")
                .about("generates a new and empty configuration file"),
                SubCommand::with_name("export-as-json")
                .about("exports the yaml config file into a json file")
            ]
        )
    )
    .subcommand(
        SubCommand::with_name("multisig")
        .about("multisig management commands")
        .subcommands(vec![
            SubCommand::with_name("new-config")
            .about("generates a new multisig config file with the given threshold, and owners")
            .arg(
                Arg::with_name("owners")
                .short("o")
                .long("owners")
                .help("the owners to be added to the multisig")
                .takes_value(true)
            )
            .arg(
                Arg::with_name("threshold")
                .short("t")
                .long("threshold")
                .help("specifies the minimum required signers")
                .takes_value(true)
                .value_name("COUNT")
            ).arg(
                Arg::with_name("name")
                .short("n")
                .long("name")
                .help("used to name the multisig account in the config file")
                .takes_value(true)
            ),  
            SubCommand::with_name("create")
            .about("create a new multisig account, requires that a config currently exists")
            .arg(
                Arg::with_name("name")
                .short("n")
                .long("name")
                .help("the name of the multisig account were creating")
                .takes_value(true)
            ),
            SubCommand::with_name("create-token-account")
            .about("create an ata for the multisig saving in file")
            .arg(
                Arg::with_name("name")
                .short("n")
                .long("name")
                .help("the name of the multisig ")
                .takes_value(true)
            )
            .arg(
                Arg::with_name("token-name")
                .short("t")
                .long("token-name")
                .help("the name of the token")
                .takes_value(true)
            )
            .arg(
                Arg::with_name("token-mint")
                .short("m")
                .long("token-mint")
                .help("the mint of  the token")
                .takes_value(true)
            ),
            SubCommand::with_name("transfer-tokens")
            .about("submit a token transfer tx to the multisig")
            .arg(
                Arg::with_name("name")
                .short("n")
                .long("name")
                .help("the name of the multisig we are submitting to")
                .takes_value(true)
            )
            .arg(
                Arg::with_name("source")
                .short("s")
                .long("source")
                .help("the source token account to transfer from")
                .takes_value(true)
                .value_name("ADDRESS")
            )
            .arg(
                Arg::with_name("target")
                .short("t")
                .long("target")
                .help("the target to transfer tokens to, this must be a wallet address")
                .takes_value(true)
                .value_name("ADDRESS")
            )
            .arg(
                Arg::with_name("amount")
                .short("a")
                .long("amount")
                .help("the amount of tokens to send, denominated in 'ui amount'")
                .takes_value(true)
                .value_name("AMOUNT")
            )
            .arg(
                Arg::with_name("decimals")
                .short("d")
                .long("decimals")
                .help("the number of decimals in the token, used for converting ui amounts to token amounts")
                .takes_value(true)
                .value_name("DECIMALS")
            ),
        ])
    )
    .get_matches();
    let config_file_path = get_config_or_default(&matches);
    let keypair = get_keypair_or_default(&matches);
    process_matches(&matches, config_file_path, keypair).await?;
    Ok(())
}

// returns the value of the config file argument or the default
fn get_config_or_default(matches: &clap::ArgMatches) -> String {
    matches
        .value_of("config")
        .unwrap_or("config.yaml")
        .to_string()
}

fn get_keypair_or_default(matches: &clap::ArgMatches) -> String {
    matches.value_of("keypair")
    .unwrap_or("usb://ledger")
    .to_string()
}

async fn process_matches<'a>(matches: &clap::ArgMatches<'a>, config_file_path: String, keypair: String) -> Result<()> {
    match matches.subcommand() {
        ("config", Some(config_command)) => match config_command.subcommand() {
            ("new", Some(new_config)) => config::new_config(new_config, config_file_path),
            ("export-as-json", Some(export_as_json)) => {
                config::export_as_json(export_as_json, config_file_path)
            }
            _ => invalid_subcommand("config"),
        },
        ("multisig", Some(multisig_command)) => match multisig_command.subcommand() {
            ("new-config", Some(new_multisig)) => {
                multisig::new_multisig_config(new_multisig, config_file_path)
            },
            ("create", Some(create_command)) => {
                multisig::create_multisig(create_command, config_file_path, keypair)
            }
            ("transfer-tokens", Some(transfer_tokens)) => {
                multisig::transfer_tokens(transfer_tokens, config_file_path, keypair)
            }
            ("create-token-account",  Some(create)) => {
                multisig::create_token_account(create, config_file_path, keypair)
            }
            _ => invalid_subcommand("multisig")
        }
        _ => invalid_command(),
    }
}

fn invalid_subcommand(command_group: &str) -> Result<()> {
    Err(anyhow!("invalid command found for group {}", command_group))
}

fn invalid_command() -> Result<()> {
    Err(anyhow!("invalid command found"))
}