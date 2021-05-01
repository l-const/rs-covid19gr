use rs_covid19gr::daily::*;
use serde::{Deserialize, Serialize};
const BASE_URL: &'static str = "https://covid-19-greece.herokuapp.com";

fn main() {
    println!("Hello from main");
    let total_tests_series = get_total_tests_series_data();
    let iter_len = total_tests_series.iter().len();
    let first_iter = total_tests_series.iter().skip(1);
    let second_iter = total_tests_series.iter().take(iter_len - 1 as usize);
    let zip_iter = first_iter.zip(second_iter);
    let iter_dates: Vec<&str> = total_tests_series
        .iter()
        .skip(1)
        .map(|x| x.date.as_str())
        .collect();
    let result: Vec<i64> = zip_iter
        .map(|(x, y)| x.tests.unwrap_or(0) as i64 - y.tests.unwrap_or(0) as i64)
        .collect();
    result
        .iter()
        .zip(iter_dates.iter())
        .for_each(|(&x, &y)| println!("Diff: {:?} Date: {:?}", x, y));
}
