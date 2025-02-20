use clap::Parser;

#[derive(Parser)]
enum Cli {
    Save,
    Load,
    Replace,
}
