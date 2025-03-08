use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "atosaki-cli")]
#[command(about = "atosaki client CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Save,
    Load,
    Replace,
}
