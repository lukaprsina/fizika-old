use std::{cell::RefCell, collections::HashMap, rc::Rc};

use uuid::Uuid;

use super::context::{Context, CreateEquationError};

#[derive(Debug)]
pub struct App {
    pub formulas: Uuid,
    pub contexts: HashMap<Uuid, Context>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<App>>, CreateEquationError> {
        let app = Rc::new(RefCell::new(App {
            formulas: Uuid::nil(),
            contexts: HashMap::new(),
        }));

        let mut borrowed_app = app.borrow_mut();

        let context = Context::new(Rc::clone(&app));
        let uuid = borrowed_app.add_context(context);

        borrowed_app.formulas = uuid;
        let context = borrowed_app.get_context_mut(uuid).unwrap();

        for line in include_str!("../../formulas.txt").lines() {
            context.try_add_equation(line)?;
        }

        std::mem::drop(borrowed_app);
        Ok(app)
    }

    pub fn add_context(&mut self, mut context: Context) -> Uuid {
        let uuid = Uuid::new_v4();
        context.uuid = uuid;
        self.contexts.insert(uuid, context);
        uuid
    }

    pub fn get_context(&self, uuid: Uuid) -> Option<&Context> {
        self.contexts.get(&uuid)
    }

    pub fn get_context_mut(&mut self, uuid: Uuid) -> Option<&mut Context> {
        self.contexts.get_mut(&uuid)
    }
}
