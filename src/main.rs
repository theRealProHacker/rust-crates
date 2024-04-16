#![allow(dead_code)]
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

/// Generates a random number from 1 to 10
fn random_example() {
    let mut range = rand::thread_rng();
    println!("{}", range.gen_range(1..=10));
}

lazy_static! {
    /// This is the static regex
    static ref EXPR: Regex = Regex::new(r"^\d{4}-\d{1,2}-\d{1,2}$").unwrap();
}

/// Prints whether the given date is a valid date or not
fn lazy_static_regex_example(date: &str) {
    if EXPR.is_match(date) {
        println!("{} is a valid date", date);
    } else {
        println!("{} is not a valid date", date);
    }
}

#[derive(Debug)]
struct Error1;
#[derive(Debug)]
struct Error2;

impl std::fmt::Display for Error1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error1")
    }
}

impl std::fmt::Display for Error2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error2")
    }
}

impl std::error::Error for Error1 {}
impl std::error::Error for Error2 {}

fn raise_error1() -> Result<(), Error1> {
    Err(Error1)
}

fn raise_error2() -> Result<(), Error2> {
    Err(Error2)
}

fn combine_errors() -> anyhow::Result<()> {
    raise_error1()?;
    raise_error2()?;
    Ok(())
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize)]
struct JSON {
    value: i32
}


fn serde_example() -> Result<()> {
    const FILE: &str = "data.json";
    let contents = std::fs::read_to_string(FILE).unwrap_or("{\"value\": 0}".to_string());

    let mut json: JSON = from_str(&contents)?;

    json.value += 1;

    std::fs::write(FILE, to_string(&json)?)?;

    Ok(())
}

async fn async_example() {
    let fut1 = reqwest::get("https://www.rust-lang.org");
    let fut2 = reqwest::get("https://tokio.rs/tokio/tutorial/hello-tokio");
    match tokio::join!(fut1, fut2) {
        (Ok(resp1), Ok(resp2)) => {
            println!("Rust lang: {:#?}", resp1.status());
            println!("Tokio: {:#?}", resp2.status());
        }
        _ => println!("There was an error"),
    }
}

#[tokio::main]
async fn main() {
    // random_example();
    // lazy_static_regex_example("2021-9-11");
    // async_example().await;
    // serde_example()
}
