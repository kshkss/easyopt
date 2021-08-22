pub mod criteria;
pub mod error;
pub mod executor;
pub mod monitor;
pub mod traits;

pub mod find_root;
pub mod self_consistent;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
