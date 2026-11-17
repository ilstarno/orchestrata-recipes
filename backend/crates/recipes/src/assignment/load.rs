use roxmltree::{Document, Node, ParsingOptions};

use super::{
    AssignmentError, AssignmentRecipeSpec, assignment_mode::AssignmentMode, skill_match::SkillMatch,
};
use crate::constants::{MAX_BYTES, MAX_IDENTIFIER_BYTES};

/// Read the bounded assignment format. The older descriptive recipe format is separate.
pub fn load_assignment_recipe(source: &str) -> Result<AssignmentRecipeSpec, AssignmentError> {
    if source.len() > MAX_BYTES {
        return Err(AssignmentError::RecipeTooLarge);
    }
    if source.contains("<!DOCTYPE") {
        return Err(AssignmentError::InvalidRecipe);
    }
    let options = ParsingOptions {
        allow_dtd: false,
        nodes_limit: 128,
        entity_resolver: None,
    };
    let doc = Document::parse_with_options(source, options)
        .map_err(|_| AssignmentError::InvalidRecipe)?;
    if doc.descendants().any(|node| node.is_pi()) {
        return Err(AssignmentError::InvalidRecipe);
    }
    let root = doc.root_element();
    check_tag(root, "assignment-recipe", &["version"])?;
    let [jobs, workers, eligibility, objective, output] = children(root)?
        .try_into()
        .map_err(|_| AssignmentError::InvalidRecipe)?;
    check_leaf(jobs, "jobs", &["field"])?;
    let mode = match root.attribute("version") {
        Some("1") => {
            check_leaf(workers, "workers", &["field", "capacities"])?;
            check_leaf(eligibility, "eligibility", &["field"])?;
            AssignmentMode::Indexed {
                capacities: field(workers, "capacities")?,
                eligibility: field(eligibility, "field")?,
            }
        }
        Some("2") => {
            check_leaf(workers, "workers", &["field"])?;
            check_leaf(eligibility, "eligibility", &["kind", "match"])?;
            if eligibility.attribute("kind") != Some("required-skill-and-available") {
                return Err(AssignmentError::InvalidRecipe);
            }
            AssignmentMode::BySkill(match eligibility.attribute("match") {
                Some("exact") => SkillMatch::Exact,
                Some("ascii-case-insensitive") => SkillMatch::AsciiCaseInsensitive,
                _ => return Err(AssignmentError::InvalidRecipe),
            })
        }
        _ => return Err(AssignmentError::InvalidRecipe),
    };
    check_leaf(objective, "objective", &["kind"])?;
    check_leaf(output, "output", &["field"])?;
    if objective.attribute("kind") != Some("maximum-coverage") {
        return Err(AssignmentError::InvalidRecipe);
    }
    let spec = AssignmentRecipeSpec {
        jobs: field(jobs, "field")?,
        workers: field(workers, "field")?,
        mode,
        output: field(output, "field")?,
    };
    let mut names = vec![&spec.jobs, &spec.workers];
    if let AssignmentMode::Indexed {
        capacities,
        eligibility,
    } = &spec.mode
    {
        names.extend([capacities, eligibility]);
    }
    if spec.output == "assigned"
        || names
            .iter()
            .enumerate()
            .any(|(i, name)| names[..i].contains(name))
    {
        return Err(AssignmentError::InvalidRecipe);
    }
    Ok(spec)
}

fn check_tag(node: Node<'_, '_>, tag: &str, attrs: &[&str]) -> Result<(), AssignmentError> {
    if !node.has_tag_name(tag)
        || node.tag_name().namespace().is_some()
        || node.namespaces().len() != 0
        || node.attributes().len() != attrs.len()
        || node
            .attributes()
            .any(|attr| attr.namespace().is_some() || !attrs.contains(&attr.name()))
    {
        return Err(AssignmentError::InvalidRecipe);
    }
    Ok(())
}

fn check_leaf(node: Node<'_, '_>, tag: &str, attrs: &[&str]) -> Result<(), AssignmentError> {
    check_tag(node, tag, attrs)?;
    if !children(node)?.is_empty() {
        return Err(AssignmentError::InvalidRecipe);
    }
    Ok(())
}

fn children<'a, 'input>(node: Node<'a, 'input>) -> Result<Vec<Node<'a, 'input>>, AssignmentError> {
    if node
        .children()
        .any(|child| child.is_text() && !child.text().unwrap_or_default().trim().is_empty())
    {
        return Err(AssignmentError::InvalidRecipe);
    }
    Ok(node.children().filter(Node::is_element).collect())
}

fn field(node: Node<'_, '_>, attr: &str) -> Result<String, AssignmentError> {
    let name = node.attribute(attr).ok_or(AssignmentError::InvalidRecipe)?;
    if name.len() > MAX_IDENTIFIER_BYTES
        || !name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(AssignmentError::InvalidRecipe);
    }
    Ok(name.to_owned())
}
