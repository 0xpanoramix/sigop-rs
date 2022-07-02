use clap::Parser;
use log::{info, warn};
use sigop_core::optimizer::run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The function signature to optimize.
    #[clap(short, long)]
    signature: String,

    /// The maximum size of the suffix following the original function name.
    #[clap(short, long, default_value_t = 3)]
    level: u8,

    /// The number of zero-bytes you want to have at the beginning of the optimized function.
    #[clap(short, long, default_value_t = 2)]
    target: u8,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let optimized = run(&cli.signature, cli.level, cli.target);

    match optimized {
        None => {
            warn!("Either none optimization was found or an error has occurred")
        }
        Some(res) => {
            info!("Found this optimization: {}", res)
        }
    }
}
