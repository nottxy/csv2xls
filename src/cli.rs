use clap::{Args, ErrorKind, IntoApp, Parser};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) enum Cli {
    Inline(CliConfig),
    File(CliFile),
}

#[derive(Debug, Args, Deserialize)]
pub(crate) struct CliConfig {
    #[clap(required = true, long, parse(from_os_str), multiple_occurrences(true))]
    pub(crate) csv: Vec<PathBuf>,
    #[clap(long, multiple_occurrences(true))]
    pub(crate) title: Vec<String>,
    #[clap(long, parse(from_os_str))]
    pub(crate) xls: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct CliFile {
    #[clap(long, parse(from_os_str))]
    file: PathBuf,
}

impl Into<CliConfig> for Cli {
    fn into(self) -> CliConfig {
        match self {
            Cli::Inline(cli_config) => return cli_config,
            Cli::File(cli_file) => match cli_file.try_into() {
                Ok(cli_config) => cli_config,
                Err(err) => {
                    let mut app = Cli::into_app();
                    app.error(ErrorKind::ValueValidation, err).exit();
                }
            },
        }
    }
}

impl TryInto<CliConfig> for CliFile {
    type Error = String;

    fn try_into(self) -> Result<CliConfig, Self::Error> {
        let file_content = fs::read_to_string(&self.file)
            .map_err(|err| format!("Read config file({:?}) ERROR: {:?}", &self.file, err))?;

        toml::from_str(&file_content)
            .map_err(|err| format!("Deserialize config file({:?}) ERROR: {:?}", &self.file, err))
    }
}
