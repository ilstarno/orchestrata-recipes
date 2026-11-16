use std::collections::BTreeMap;

use crate::{
    FieldKind, FieldSpec, FieldValue, PayloadError,
    constants::{
        MAX_FIELDS, MAX_IDENTIFIER_BYTES, MAX_PAYLOAD_BYTES, MAX_VALUE_LIST_ITEMS,
        MAX_VALUE_TEXT_BYTES,
    },
};

pub(super) fn validate(
    fields: &[FieldSpec],
    values: &BTreeMap<String, FieldValue>,
) -> Result<(), PayloadError> {
    if values.len() > MAX_FIELDS {
        return Err(PayloadError::TooLarge);
    }
    let mut bytes = 0;
    for (name, value) in values {
        if name.len() > MAX_IDENTIFIER_BYTES {
            return Err(PayloadError::TooLarge);
        }
        let field = fields
            .iter()
            .find(|field| &field.name == name)
            .ok_or_else(|| PayloadError::UnknownField(name.clone()))?;
        bytes += value_size(field, value)?;
        if bytes > MAX_PAYLOAD_BYTES {
            return Err(PayloadError::TooLarge);
        }
    }
    for field in fields {
        if field.required && !values.contains_key(&field.name) {
            return Err(PayloadError::MissingField(field.name.clone()));
        }
    }
    Ok(())
}

fn value_size(field: &FieldSpec, value: &FieldValue) -> Result<usize, PayloadError> {
    match (field.kind, value) {
        (FieldKind::String, FieldValue::String(text)) => text_size(text),
        (FieldKind::Integer, FieldValue::Integer(_)) => Ok(size_of::<i64>()),
        (FieldKind::Boolean, FieldValue::Boolean(_)) => Ok(size_of::<bool>()),
        (FieldKind::StringList, FieldValue::StringList(items)) => {
            list_size(items.len())?;
            items.iter().try_fold(0, |sum, item| {
                let size = sum + text_size(item)?;
                if size > MAX_PAYLOAD_BYTES {
                    return Err(PayloadError::TooLarge);
                }
                Ok(size)
            })
        }
        (FieldKind::IntegerList, FieldValue::IntegerList(items)) => {
            list_size(items.len())?;
            Ok(items.len() * size_of::<i64>())
        }
        _ => Err(PayloadError::WrongType(field.name.clone())),
    }
}

fn text_size(text: &str) -> Result<usize, PayloadError> {
    if text.len() > MAX_VALUE_TEXT_BYTES {
        return Err(PayloadError::TooLarge);
    }
    Ok(text.len())
}

fn list_size(size: usize) -> Result<(), PayloadError> {
    if size > MAX_VALUE_LIST_ITEMS {
        return Err(PayloadError::TooLarge);
    }
    Ok(())
}
