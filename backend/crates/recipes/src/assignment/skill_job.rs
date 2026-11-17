use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SkillJob {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) required_skill: String,
}
