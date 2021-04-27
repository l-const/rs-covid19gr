//! Rust consumer/client for the Covid19gr Api!
//! A simple and fast Api for the coronavirus outbreak
mod cum;
mod daily;
mod hosp;
mod refug;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
