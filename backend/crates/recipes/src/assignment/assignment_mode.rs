use super::skill_match::SkillMatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum AssignmentMode {
    Indexed {
        capacities: String,
        eligibility: String,
    },
    BySkill(SkillMatch),
}
