//! Rust bindings  for the Covid19gr API!
pub mod cum;
pub mod daily;
pub mod hosp;
pub mod refug;

// pub use daily::*;
// pub use refug::*;
// pub use cum::*;
// pub use hosp::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
