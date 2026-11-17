/// Names the supplied fields and where the assignment is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentRecipeSpec {
    pub(super) jobs: String,
    pub(super) workers: String,
    pub(super) capacities: String,
    pub(super) eligibility: String,
    pub(super) output: String,
}

impl AssignmentRecipeSpec {
    pub fn output_field(&self) -> &str {
        &self.output
    }
}
