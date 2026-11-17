use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentError {
    RecipeTooLarge,
    InvalidRecipe,
    InputsTooLarge,
    InvalidInputs,
    MissingField(String),
    InvalidField(String),
    UnexpectedFields,
    TooManyRecords,
}

impl fmt::Display for AssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecipeTooLarge => f.write_str("Use an assignment recipe no larger than 32 KiB."),
            Self::InvalidRecipe => f.write_str(
                "Use a version 1 assignment recipe with maximum coverage and named fields.",
            ),
            Self::InputsTooLarge => f.write_str("Use assignment inputs no larger than 512 KiB."),
            Self::InvalidInputs => {
                f.write_str("Supply one JSON object without repeated field names.")
            }
            Self::MissingField(field) => write!(f, "Supply the {field} field named in the recipe."),
            Self::InvalidField(field) => write!(f, "Check the values in {field}."),
            Self::TooManyRecords => f.write_str("Use at most 200 jobs and workers combined."),
            Self::UnexpectedFields => f.write_str("Use only the input fields named in the recipe."),
        }
    }
}

impl Error for AssignmentError {}
