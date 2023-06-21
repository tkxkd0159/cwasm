pub mod contract;
mod error;
pub use crate::error::ContractError;
pub mod msg;
mod query;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
