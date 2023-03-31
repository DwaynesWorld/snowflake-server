#[macro_use]
extern crate error_chain;
extern crate dotenv;

mod config;
mod errors;
mod generator;
mod logger;
mod server;
mod worker;

use clap::Parser;
use clap::Subcommand;
use dotenv::dotenv;
use log::error;

use config::ServerConfig;

const INFO: &str = "API server that generates Snowflake IDs.";

#[derive(Debug, Parser)]
#[clap(name = "API service command-line interface")]
#[clap(about = INFO, disable_version_flag = true, arg_required_else_help = true)]
struct AppOptions {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Server(ServerConfig),
}

#[actix_web::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let app = AppOptions::parse();

    let output = match app.command {
        Commands::Server(c) => server::run(c).await,
    };

    if let Err(e) = output {
        error!("{}", e);
    }
}
