use crate::cli::{Cli, CliConfig};
use clap::Parser;

mod cli;

fn main() {
    let cli_config: CliConfig = Cli::parse().into();

    if let Err(err) = csv2xls::convert(&cli_config.title, &cli_config.csv, &cli_config.xls) {
        eprintln!(
            "csv2xls convert error: {}, cli_config: {:?}",
            err, &cli_config
        );
    } else {
        println!(
            "csv2xls convert success: {}",
            &cli_config.xls.to_str().unwrap_or_default()
        );
    }
}
