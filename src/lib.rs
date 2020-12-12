mod sys;
use crate::sys::Selector;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod token;
pub use self::token::*;
mod poll;
pub use self::poll::*;