use clap::Parser;
use env_logger::{Builder, Env};
use monotax::cli::app::run_cli_command;
use monotax::cli::Cli;
use monotax::db::sqlx::connection::default_connection_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read local .env file
    let _ = dotenvy::dotenv();
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let db_pool = default_connection_pool().await?;
    let cli = Cli::parse();
    run_cli_command(&cli, db_pool).await
}
