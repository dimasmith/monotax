use std::process::exit;

use clap::Parser;
use cli::router;
use cli::Cli;
use config::load_config;
use env_logger::{Builder, Env};
use log::{debug, error};
use monotax_sqlite::{configuration::DatabaseConfiguration, connection::connection_pool};
use sqlx::{Pool, Sqlite};

mod cli;
mod config;
mod init;

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
    let db_pool = connect_to_database(config.database()).await;

    router::handle_command(&cli, &config, db_pool).await
}

async fn connect_to_database(db_config: &DatabaseConfiguration) -> Pool<Sqlite> {
    match connection_pool(db_config).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("monotax cannot connect to a database on {}.", db_config.url);
            eprintln!("consider using `monotax init` to create a database.");
            error!("database connection failed. {:?}", e);
            exit(-1);
        }
    }
}
