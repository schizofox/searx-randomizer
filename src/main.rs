mod util;

extern crate clap;
extern crate rouille;

use clap::Parser;
use rouille::router;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
struct Opts {
    /// IP address to bind the server
    #[clap(short = 'i', long, default_value = "127.0.0.1")]
    ip: String,

    /// Port to bind the server
    #[clap(short = 'p', long, default_value = "8000")]
    port: String,

    /// Path to the JSON file containing Searx instances
    #[clap(short = 'f', long, env = "SEARX_INSTANCES")]
    searx_instances: String,
}

fn main() {
    let opts = Opts::parse();
    let mut json = String::new();
    if let Err(e) =
        File::open(&opts.searx_instances).and_then(|mut file| file.read_to_string(&mut json))
    {
        eprintln!("Failed to read file '{}': {}", opts.searx_instances, e);
        std::process::exit(1);
    }

    let engines: Vec<String> = match serde_json::from_str(&json) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
            std::process::exit(1);
        }
    };

    if engines.is_empty() {
        eprintln!("The list of search engines is empty. Please check the input file.");
        std::process::exit(1);
    }

    let address = format!("{}:{}", opts.ip, opts.port);
    println!("Now listening on {}", address);

    rouille::start_server(address, move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("Incorrect endpoint! Please use /search!")
            },
            (GET) (/search) => {
                let engine = util::get_random_element(&engines);
                rouille::Response::redirect_302(format!(
                    "https://{}/search?{}",
                    engine,
                    request.raw_query_string()
                ))
            },
            _ => rouille::Response::empty_404()
        )
    });
}
