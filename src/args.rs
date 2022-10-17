use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, value_name = "tx_id")]
    pub transaction: String,

    #[arg(long, value_name = "file_name")]
    pub output: String,

    #[arg(long, value_name = "num", default_value = "10")]
    pub connections: usize,
}

pub fn parse() -> Args {
    Args::parse()
}
