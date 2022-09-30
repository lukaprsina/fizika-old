use std::collections::HashMap;

use color_eyre::Result;
use itertools::Itertools;
use select::{
    document::Document,
    node::Node,
    predicate::{And, Class},
};
use thiserror::Error;
use uuid::Uuid;

use crate::utils::get_only_element;

pub fn process_popup<'a, 'b>(document: &'a Document) -> Result<(String, Node<'b>)>
where
    'a: 'b,
{
    let areas = document.find(Class("popupContent")).collect_vec();
    let area = get_only_element(areas);

    let exercises = document
        .find(And(Class("eplxSlide"), Class("popupImpl")))
        .collect_vec();
    let exercise = get_only_element(exercises);
    let uuid = exercise.attr("id").unwrap();

    // TODO: pri kvizu so za naprej popupi v popupu

    Ok((uuid.to_string(), area))
}

#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum ExerciseError {
    #[error("This is a hidden exercise")]
    HiddenExercise,
}

pub fn process_exercise<'a, 'b>(
    document: &'a Document,
) -> Result<(HashMap<String, Uuid>, Option<Node<'b>>), ExerciseError>
where
    'a: 'b,
{
    let exercises = document.find(Class("eplxSlide")).collect_vec();
    let exercise = get_only_element(exercises);

    let area = if exercise.is(Class("popupImpl")) {
        return Err(ExerciseError::HiddenExercise);
    } else if exercise.is(Class("eplxLastSlide")) {
        None
    } else {
        let subheadings = exercise.find(Class("subheading")).collect_vec();
        if subheadings.is_empty() {
            unreachable!("{}\n", exercise.html());
        }
        let _subheading = get_only_element(subheadings);

        let areas = document.find(Class("interactive-area")).collect_vec();
        let area = get_only_element(areas);

        Some(area)
    };

    Ok((HashMap::new(), area))
}
