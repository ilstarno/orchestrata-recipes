use std::collections::HashSet;

use roxmltree::{Document, Node, ParsingOptions};

use crate::{
    FieldKind, FieldSpec, Recipe, RecipeError,
    constants::{
        MAX_BYTES, MAX_DESCRIPTION_CHARS, MAX_FIELDS, MAX_IDENTIFIER_BYTES, MAX_NODES,
        MAX_TITLE_CHARS,
    },
};

pub fn load_recipe(source: &str) -> Result<Recipe, RecipeError> {
    if source.len() > MAX_BYTES {
        return Err(RecipeError::TooLarge);
    }
    // Reject even an empty DTD, which the XML parser permits when DTDs are disabled.
    if source.contains("<!DOCTYPE") {
        return Err(RecipeError::ForbiddenDeclaration);
    }

    let options = ParsingOptions {
        allow_dtd: false,
        nodes_limit: MAX_NODES,
        entity_resolver: None,
    };
    let doc =
        Document::parse_with_options(source, options).map_err(|_| RecipeError::MalformedXml)?;
    if doc.descendants().any(|node| node.is_pi()) {
        return Err(RecipeError::ForbiddenDeclaration);
    }
    let root = doc.root_element();
    expect_tag(root, "recipe", &["version", "id"])?;
    if root.attribute("version") != Some("1") {
        return Err(RecipeError::UnsupportedVersion);
    }

    let id = identifier(attribute(root, "id")?)?;
    let [title, scenario, inputs, outputs] = element_children(root)?
        .try_into()
        .map_err(|_| RecipeError::InvalidStructure)?;
    Ok(Recipe {
        id,
        title: text(title, "title", MAX_TITLE_CHARS)?,
        scenario: text(scenario, "scenario", MAX_DESCRIPTION_CHARS)?,
        inputs: fields(inputs, "inputs")?,
        outputs: fields(outputs, "outputs")?,
    })
}

fn fields(node: Node<'_, '_>, name: &str) -> Result<Vec<FieldSpec>, RecipeError> {
    expect_tag(node, name, &[])?;
    let children = element_children(node)?;
    if !(1..=MAX_FIELDS).contains(&children.len()) {
        return Err(RecipeError::InvalidFieldCount);
    }

    let mut names = HashSet::with_capacity(children.len());
    let mut result = Vec::with_capacity(children.len());
    for field in children {
        expect_tag(field, "field", &["name", "type", "required"])?;
        let name = attribute(field, "name")?;
        if !names.insert(name) {
            return Err(RecipeError::DuplicateField);
        }
        let [description] = element_children(field)?
            .try_into()
            .map_err(|_| RecipeError::InvalidStructure)?;
        result.push(FieldSpec {
            name: identifier(name)?,
            kind: field_kind(attribute(field, "type")?)?,
            required: required_flag(attribute(field, "required")?)?,
            description: text(description, "description", MAX_DESCRIPTION_CHARS)?,
        });
    }
    Ok(result)
}

fn field_kind(value: &str) -> Result<FieldKind, RecipeError> {
    match value {
        "string" => Ok(FieldKind::String),
        "integer" => Ok(FieldKind::Integer),
        "boolean" => Ok(FieldKind::Boolean),
        "string-list" => Ok(FieldKind::StringList),
        "integer-list" => Ok(FieldKind::IntegerList),
        _ => Err(RecipeError::InvalidFieldKind),
    }
}

fn required_flag(value: &str) -> Result<bool, RecipeError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(RecipeError::InvalidRequiredFlag),
    }
}

fn attribute<'a>(node: Node<'a, '_>, name: &str) -> Result<&'a str, RecipeError> {
    node.attribute(name).ok_or(RecipeError::InvalidStructure)
}

fn expect_tag(node: Node<'_, '_>, name: &str, attributes: &[&str]) -> Result<(), RecipeError> {
    if node.tag_name().name() != name
        || node.tag_name().namespace().is_some()
        || node.namespaces().len() != 0
        || node
            .attributes()
            .any(|attr| attr.namespace().is_some() || !attributes.contains(&attr.name()))
    {
        return Err(RecipeError::InvalidStructure);
    }
    Ok(())
}

fn element_children<'a, 'input>(
    node: Node<'a, 'input>,
) -> Result<Vec<Node<'a, 'input>>, RecipeError> {
    let mut elements = Vec::new();
    for child in node.children() {
        if child.is_element() {
            elements.push(child);
        } else if child.is_text() && !child.text().is_some_and(|value| value.trim().is_empty()) {
            return Err(RecipeError::InvalidStructure);
        }
    }
    Ok(elements)
}

fn identifier(value: &str) -> Result<String, RecipeError> {
    if value.len() > MAX_IDENTIFIER_BYTES
        || !value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(RecipeError::InvalidIdentifier);
    }
    Ok(value.to_owned())
}

fn text(node: Node<'_, '_>, name: &str, limit: usize) -> Result<String, RecipeError> {
    expect_tag(node, name, &[])?;
    if node.children().any(|child| !child.is_text()) {
        return Err(RecipeError::InvalidStructure);
    }
    let value = node.text().unwrap_or_default().trim();
    if value.is_empty() || value.chars().count() > limit {
        return Err(RecipeError::InvalidText);
    }
    Ok(value.to_owned())
}
