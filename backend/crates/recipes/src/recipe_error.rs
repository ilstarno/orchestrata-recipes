#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeError {
    TooLarge,
    ForbiddenDeclaration,
    MalformedXml,
    InvalidStructure,
    UnsupportedVersion,
    InvalidIdentifier,
    InvalidText,
    InvalidFieldKind,
    InvalidRequiredFlag,
    DuplicateField,
    InvalidFieldCount,
}
