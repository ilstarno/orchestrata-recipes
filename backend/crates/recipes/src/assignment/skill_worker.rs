use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SkillWorker {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) skills: Vec<String>,
    pub(super) available: bool,
    pub(super) capacity: u64,
}
