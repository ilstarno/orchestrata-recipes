#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadError {
    TooLarge,
    UnknownField(String),
    MissingField(String),
    WrongType(String),
}
