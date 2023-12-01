extern crate rouille;

use rand::Rng;
use rouille::{router, Response};
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

    println!("Now listening on 127.0.0.1:7482");

    rouille::start_server("127.0.0.1:7482", move |request| {
            router!(request,
                    (GET) (/) => {
                let file = match File::open("public/index.html") {
                    Ok(f) => f,
                    Err(_) => return Response::empty_404(),
                };

                Response::from_file("text/html", file)
            },
                (GET) (/search) => {
                    rouille::Response::redirect_302(format!("https://{}/search?{}", get_random_element(&engines), &request.raw_query_string()))

                },
                _ => rouille::match_assets(request, "public")
            )
    });
}
