use crate::ast::{analyzed_expression::AnalyzedElement, Equation};

pub trait Simplify {
    fn simplify(&mut self);
}

impl Simplify for Equation {
    fn simplify(&mut self) {
        let mut borrowed_app = self.app.borrow_mut();
        let context = borrowed_app.get_context_mut(self.context).unwrap();

        for uuid in self.uuids.iter() {
            let analyzed_element = context.get_expression_mut(*uuid).unwrap();
        }
    }
}

impl Simplify for AnalyzedElement {
    fn simplify(&mut self) {
        todo!()
    }
}
