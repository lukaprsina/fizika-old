#![allow(dead_code, unused_variables)]
use crate::ast::{product::Product, Element};

pub trait Bind {
    fn bind(&self, instructions: &Self);
}

impl Bind for Element {
    fn bind(&self, instructions: &Element) {}
}

impl Bind for Product {
    fn bind(&self, instructions: &Self) {}
}
