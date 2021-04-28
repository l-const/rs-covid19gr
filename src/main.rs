extern crate serde;
extern crate serde_json;
extern crate ureq;

use serde::{Deserialize, Serialize};

const BASE_URL: &'static str = "https://covid-19-greece.herokuapp.com";

#[derive(Deserialize, Serialize, Debug)]
struct Deaths {
    cases: Vec<DeathSlot>,
}

#[derive(Deserialize, Serialize,Debug)]
struct DeathSlot {
    deaths: u32,
    date: String,
}

fn main() -> Result<(), ureq::Error> {
    println!("Hello from main");
    let body: String = ureq::get(format!("{}/deaths", BASE_URL).as_str())
        .set("Accept", "application/json")
        .call()?
        .into_string()?;
    // println!("Request body: {:?}", &body);
    let deaths_res: Deaths = serde_json::from_str(&body).unwrap();

    println!("{:?}", &deaths_res.cases[0]);
    Ok(())
}
