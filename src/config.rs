use crate::logger::Level;
use clap::Args;

#[derive(Args, Debug)]
pub struct ServerConfig {
    #[clap(
        short,
        long,
        env = "LOG",
        default_value = "info",
        forbid_empty_values = true,
        help = "The logging level",
        value_enum
    )]
    /// The logging level
    pub log: Level,

    #[clap(
        long = "host",
        env = "HOST",
        default_value = "localhost",
        forbid_empty_values = true,
        help = "Host the server will bind to"
    )]
    /// Host where server will bind to
    pub host: String,

    #[clap(
        long = "port",
        env = "PORT",
        default_value = "5000",
        forbid_empty_values = true,
        help = "Port the server will bind to"
    )]
    /// Port where server will bind to
    pub port: u16,

    #[clap(
        long = "etcd-host",
        env = "ETCD_HOST",
        default_value = "localhost",
        forbid_empty_values = true,
        help = "Etcd database host the server will connect to"
    )]
    /// Etcd database host the server will connect to
    pub etcd_host: String,

    #[clap(
        long = "etcd-port",
        env = "ETCD_PORT",
        default_value = "2379",
        forbid_empty_values = true,
        help = "Etcd port where server will connect to"
    )]
    /// Etcd port where server will connect to
    pub etcd_port: u16,
}

impl ServerConfig {
    pub fn get_database_addr(&self) -> String {
        format!("{}:{}", self.etcd_host, self.etcd_port)
    }
}
