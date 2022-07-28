use crate::ast::Equation;

pub trait Simplify {
    fn simplify(&mut self);
}

impl Simplify for Equation {
    fn simplify(&mut self) {
        let mut borrowed_app = self.app.borrow_mut();
        let context = borrowed_app.get_context_mut(self.context).unwrap();

        for uuid in self.uuids.iter() {
            let expression = context.get_expression_mut(*uuid).unwrap();
        }
    }
}
