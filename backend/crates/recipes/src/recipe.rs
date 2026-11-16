use std::collections::BTreeMap;

use crate::{FieldSpec, FieldValue, PayloadError, payload};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) scenario: String,
    pub(crate) inputs: Vec<FieldSpec>,
    pub(crate) outputs: Vec<FieldSpec>,
}

impl Recipe {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn scenario(&self) -> &str {
        &self.scenario
    }

    pub fn inputs(&self) -> &[FieldSpec] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[FieldSpec] {
        &self.outputs
    }

    pub fn validate_inputs(
        &self,
        values: &BTreeMap<String, FieldValue>,
    ) -> Result<(), PayloadError> {
        payload::validate(&self.inputs, values)
    }

    pub fn validate_outputs(
        &self,
        values: &BTreeMap<String, FieldValue>,
    ) -> Result<(), PayloadError> {
        payload::validate(&self.outputs, values)
    }
}
