use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use thiserror::Error;
use uuid::Uuid;

use crate::{ast::element::ElementCache, tokenizer::parser::ParseError};

use super::{app::App, token_to_element::TokensToEquationError, Equation, Node, NodeOrExpression};

#[derive(Debug, Clone)]
pub enum Domain {}

#[derive(Debug, Clone)]
pub enum FunctionProperty {
    Idempotent,
    Involution,
    Asociative,
    Commutative,
}

#[derive(Debug, Clone)]
pub enum ElementDefinition {
    Variable {
        domain: Domain,
    },
    Function {
        domain: Domain,
        codomain: Domain,
        properties: Vec<FunctionProperty>,
    },
}

#[derive(Debug, Clone)]
pub struct Context {
    pub app: Rc<RefCell<App>>,
    equations: HashMap<Uuid, Equation>,
    pub definitions: HashMap<String, ElementDefinition>,
    pub uuid: Uuid,
}

#[derive(Debug, Error)]
pub enum CreateEquationError {
    #[error("{0}")]
    ParseError(ParseError),
    #[error("{0}")]
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new(app: Rc<RefCell<App>>) -> Context {
        Context {
            equations: HashMap::new(),
            app,
            uuid: Uuid::nil(),
            definitions: HashMap::new(),
        }
    }

    pub fn get_equation(&self, uuid: Uuid) -> Option<&Equation> {
        self.equations.get(&uuid)
    }

    pub fn get_equation_mut(&mut self, uuid: Uuid) -> Option<&mut Equation> {
        let mut equation = self.equations.get_mut(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub fn remove_equation(&mut self, uuid: Uuid) -> Option<Equation> {
        let mut equation = self.equations.remove(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub(crate) fn insert_equation(&mut self, equation: Equation) -> Uuid {
        let uuid = Uuid::new_v4();
        self.equations.insert(uuid, equation);
        uuid
    }

    pub fn analyze(&mut self) -> ContextAnalysis {
        let mut analysis = ContextAnalysis::new();

        for (_, equation) in &mut self.equations {
            for element in &mut equation.eq_sides {
                element.apply_to_every_element_mut(
                    &mut |elem| {
                        let mut nested_elem_cache = ElementCache::new();

                        if elem.cache.is_none() {
                            elem.cache = Some(ElementCache::new());
                        }

                        let cache = elem.cache.as_mut().expect("No element cache");

                        if let NodeOrExpression::Node(node) = &mut elem.node_or_expression {
                            match node {
                                Node::Function { name, arguments: _ } => {
                                    analysis.functions.insert(name.clone(), None);
                                    cache.functions.insert(name.clone());
                                }
                                Node::Variable(name) => {
                                    analysis.variables.insert(name.clone(), None);
                                    cache.variables.insert(name.clone());
                                }
                                _ => (),
                            }
                        }

                        elem.apply_to_every_element_mut(
                            &mut |elem_inner| {
                                // println!("{:#?}", elem_inner);
                                if let Some(cache) = &mut elem_inner.cache {
                                    // println!("{:#?}", cache);
                                    nested_elem_cache.functions.extend(cache.functions.clone());
                                    nested_elem_cache.variables.extend(cache.variables.clone());
                                }
                            },
                            false,
                            Some(1),
                        );

                        if let Some(cache) = elem.cache.as_mut() {
                            cache.functions.extend(nested_elem_cache.functions.clone());
                            cache.variables.extend(nested_elem_cache.variables.clone());
                        }
                    },
                    false,
                    None,
                );
            }
        }

        analysis
    }

    pub fn solve(&mut self) {
        // println!("Context {}", self.uuid);

        let analysis = self.analyze();

        // println!("Analysis: {:#?}", analysis);
    }
}

#[derive(Debug)]
pub enum VariableType {
    Constant,
    Independet,
    Dependent,
}

#[derive(Debug)]
pub enum FunctionType {
    BuiltIn,
    Custom,
}

#[derive(Default, Debug)]
pub struct ContextAnalysis {
    variables: HashMap<String, Option<VariableType>>,
    functions: HashMap<String, Option<FunctionType>>,
}

impl ContextAnalysis {
    pub fn new() -> ContextAnalysis {
        ContextAnalysis {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}
