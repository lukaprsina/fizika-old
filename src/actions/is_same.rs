use std::collections::HashSet;

use uuid::Uuid;

use crate::ast::{app::App, Equation};

pub trait IsSame {
    fn is_same(lhs: &Self, rhs: &Self) -> bool;
    fn preprocess_cycles(&self) -> HashSet<std::ptr::NonNull<usize>>;
    fn same_impl(set: &mut HashSet<std::ptr::NonNull<usize>>);
    fn test(lhs: &Self, rhs: &Self) -> bool;
}

impl<T: Ord + Clone> IsSame for Vec<T> {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        let mut a = (*lhs).clone();
        let mut b = (*rhs).clone();
        a.sort();
        b.sort();
        a == b
    }

    fn preprocess_cycles(&self, set: &mut HashSet<std::ptr::NonNull<usize>>) {}

    fn same_impl(set: &mut HashSet<std::ptr::NonNull<usize>>) {}

    fn test(lhs: &Self, rhs: &Self) -> bool {
        false
    }
}

impl IsSame for Equation {
    unsafe fn is_same(lhs: &Self, rhs: &Self) -> bool {
        let set: HashSet<std::ptr::NonNull<usize>> = HashSet::new();

        set.insert(std::mem::transmute::<&App, std::ptr::NonNull<usize>>(
            &lhs.app,
        ));

        set.insert(std::mem::transmute::<&Uuid, std::ptr::NonNull<usize>>(
            &lhs.context,
        ));

        false
    }
}
