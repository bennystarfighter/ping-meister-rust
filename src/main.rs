pub mod config;
pub mod handlers;

use clap::Parser;
use handlers::Response;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let targets: Vec<config::Target> = config::read_config(args.config);

    let mut responses: Vec<(config::Target, Response)> = Vec::new();

    for target in targets {
        let resp = target.run_query();

        responses.push((target, resp));
    }

    println!("[Ping-Meister -----------------]");

    for response in responses {
        println!(
            "Name: {:}, Status: {:?}, Latency: {:?}", 
            response.0.name, response.1.status, response.1.latency
        );
    }
}
