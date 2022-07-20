use crate::ast::Element;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BindResult {
    Multiply(Element),
    Ok,
}

pub trait Match {
    type NonFree;
    fn bind(&self, other: &Self::NonFree) -> BindResult;
}
