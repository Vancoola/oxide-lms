pub mod model;
pub mod dto;
pub mod course;
pub mod error;
pub mod user;
pub mod crypto;
pub mod profile;
mod student;
mod organizational_unit;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
