#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetBenchmarkError {
    TooLarge,
    ForbiddenDeclaration,
    MalformedXml,
    InvalidStructure,
    MissingAttribute(&'static str),
    UnsupportedVersion,
    InvalidIdentifier,
    InvalidCity,
    InvalidNumber(&'static str),
    OutOfRange(&'static str),
}
