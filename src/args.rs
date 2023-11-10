use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Args {
    /// Log level [off|error|warn|info|debug|trace]
    #[clap(long, short = 'l', default_value = "info")]
	pub log_level: log::LevelFilter,

	/// Config file path (default file will be created if it does not exist)
	#[clap(long, short = 'c', default_value = "config.json")]
	pub config_path: PathBuf,
}

impl Args {
	pub fn parse() -> Self {
		Parser::parse()
	}
}