#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinRecipe {
    pub(crate) id: &'static str,
    pub(crate) xml: &'static str,
    pub(crate) problem_json: &'static str,
    pub(crate) example_path: &'static str,
}

impl BuiltinRecipe {
    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn xml(&self) -> &'static str {
        self.xml
    }

    pub fn problem_json(&self) -> &'static str {
        self.problem_json
    }

    /// Path to the included JSON example relative to the checkout root.
    pub fn example_path(&self) -> &'static str {
        self.example_path
    }
}
