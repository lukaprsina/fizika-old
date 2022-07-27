pub trait IsSame {
    fn is_same(lhs: &Self, rhs: &Self) -> bool;
}

impl<T: Ord + Clone> IsSame for Vec<T> {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        let mut a = (*lhs).clone();
        let mut b = (*rhs).clone();
        a.sort();
        b.sort();
        a == b
    }
}
