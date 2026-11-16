#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    StringList(Vec<String>),
    IntegerList(Vec<i64>),
}
