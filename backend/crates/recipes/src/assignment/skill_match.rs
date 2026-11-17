#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SkillMatch {
    Exact,
    AsciiCaseInsensitive,
}

impl SkillMatch {
    pub(super) fn key(self, skill: &str) -> std::borrow::Cow<'_, str> {
        match self {
            Self::AsciiCaseInsensitive if skill.bytes().any(|b| b.is_ascii_uppercase()) => {
                std::borrow::Cow::Owned(skill.to_ascii_lowercase())
            }
            _ => std::borrow::Cow::Borrowed(skill),
        }
    }
}
