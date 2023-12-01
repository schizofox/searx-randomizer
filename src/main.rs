extern crate rouille;

use rand::Rng;
use rouille::router;
use std::fs::File;
use std::io::Read;

fn get_random_element<T>(vector: &Vec<T>) -> &T {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..vector.len());
    &vector[random_index]
}

fn main() {
    let mut json = String::new();
    File::open(std::env::var("SEARX_INSTANCES").expect("missing SEARX_INSTANCES"))
        .expect("Failed to open file")
        .read_to_string(&mut json)
        .expect("Failed to read file");

    let engines: Vec<String> = serde_json::from_str(&json).expect("Failed to deserialize JSON");

    println!("Now listening on 127.0.0.1:8000");

    rouille::start_server("127.0.0.1:8000", move |request| {
        router!(request,
            (GET) (/search) => {
                rouille::Response::redirect_302(format!("https://{}/search?{}", get_random_element(&engines), &request.raw_query_string()))

            },
            _ => rouille::Response::empty_404()
        )
    });
}
