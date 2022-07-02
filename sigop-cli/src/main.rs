use clap::Parser;
use sigop_core::optimizer::run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    signature: String,

    #[clap(short, long, default_value_t = 3)]
    level: u8,

    #[clap(short, long, default_value_t = 2)]
    target: u8,
}

fn main() {
    let cli = Cli::parse();
    let optimized = run(&cli.signature, cli.level, cli.target);

    match optimized {
        None => {
            println!("Either none optimization was found or an error has occurred")
        }
        Some(res) => {
            println!("Found this optimization: {}", res)
        }
    }
}
