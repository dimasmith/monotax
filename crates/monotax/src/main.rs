use std::process::exit;

use clap::Parser;
use env_logger::{Builder, Env};
use log::{debug, error};
use monotax_core::cli::app::run_cli_command;
use monotax_core::cli::Cli;
use monotax_core::config::load_config;
use monotax_core::infra::sqlx::connection::connection_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read local .env file
    let _ = dotenvy::dotenv();
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let config = load_config()?;
    debug!("startup configuration: {:?}", config);

    let cli = Cli::parse();
    let db_pool = match connection_pool(&config.database).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!(
                "monotax cannot connect to a database on {}.",
                config.database.url
            );
            eprintln!("consider using `monotax init` to create a database.");
            error!("database connection failed. {:?}", e);
            exit(-1);
        }
    };

    run_cli_command(&cli, &config, db_pool).await
}
