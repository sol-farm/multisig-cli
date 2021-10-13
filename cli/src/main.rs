#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
use anyhow::{Result, anyhow};
use tokio;
use clap::{App, Arg, SubCommand};
mod config;

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
    .get_matches();
    let config_file_path = get_config_or_default(&matches);
    process_matches(&matches, config_file_path).await?;
    Ok(())
}

// returns the value of the config file argument or the default
fn get_config_or_default(matches: &clap::ArgMatches) -> String {
    matches
        .value_of("config")
        .unwrap_or("config.yaml")
        .to_string()
}

async fn process_matches<'a>(matches: &clap::ArgMatches<'a>, config_file_path: String) -> Result<()> {
    match matches.subcommand() {
        ("config", Some(config_command)) => match config_command.subcommand() {
            ("new", Some(new_config)) => config::new_config(new_config, config_file_path),
            ("export-as-json", Some(export_as_json)) => {
                config::export_as_json(export_as_json, config_file_path)
            }
            _ => invalid_subcommand("config"),
        },
        _ => invalid_command(),
    }
}

fn invalid_subcommand(command_group: &str) -> Result<()> {
    Err(anyhow!("invalid command found for group {}", command_group))
}

fn invalid_command() -> Result<()> {
    Err(anyhow!("invalid command found"))
}