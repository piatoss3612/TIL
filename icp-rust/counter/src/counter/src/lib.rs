use std::cell::RefCell;

use candid::Nat;
use ic_cdk::{query, update};

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

#[query]
fn get() -> Nat {
    COUNTER.with(|c| (*c.borrow()).clone())
}

#[update]
fn set(n: Nat) {
    COUNTER.with(|c| *c.borrow_mut() = n);
}

#[update]
fn inc() {
    COUNTER.with(|c| *c.borrow_mut() += Nat::from(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert_eq!(get(), Nat::from(0));
    }

    #[test]
    fn test_inc() {
        inc();
        assert_eq!(get(), Nat::from(1));
    }

    #[test]
    fn test_set() {
        set(Nat::from(42));
        assert_eq!(get(), Nat::from(42));
    }
}
