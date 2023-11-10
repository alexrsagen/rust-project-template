mod alloc;
mod args;
mod config;
mod logger;

use anyhow::Result;

#[global_allocator]
static GLOBAL: alloc::SystemTrackingAllocator = alloc::SystemTrackingAllocator::new_system();

#[tokio::main]
async fn main() -> Result<()> {
    // parse command-line arguments
    let args = args::Args::parse();

    // set up logger
    logger::try_init(args.log_level)?;

    // load or create default config
    let config = config::Config::load_or_init(&args.config_path)?;

	Ok(())
}
