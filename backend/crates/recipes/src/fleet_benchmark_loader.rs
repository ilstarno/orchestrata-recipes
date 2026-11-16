use roxmltree::{Document, Node, ParsingOptions};

use crate::{
    CourierMix, FleetBenchmark, FleetBenchmarkError,
    constants::MAX_IDENTIFIER_BYTES,
    fleet_benchmark_constants::{
        MAX_BYTES, MAX_CITY_CHARS, MAX_COURIERS, MAX_SHIFT_MINUTES, MAX_STOPS, MAX_XML_NODES,
        MIN_SHIFT_MINUTES,
    },
};

const ROOT_ATTRIBUTES: &[&str] = &["version", "id", "city", "stops", "shift-minutes", "seed"];
const COURIER_ATTRIBUTES: &[&str] = &["vehicle", "bicycle", "walking"];

/// Loads the small, separate version-1 benchmark descriptor format.
pub fn load_fleet_benchmark(source: &str) -> Result<FleetBenchmark, FleetBenchmarkError> {
    if source.len() > MAX_BYTES {
        return Err(FleetBenchmarkError::TooLarge);
    }
    if source.contains("<!DOCTYPE") {
        return Err(FleetBenchmarkError::ForbiddenDeclaration);
    }

    let options = ParsingOptions {
        allow_dtd: false,
        nodes_limit: MAX_XML_NODES,
        entity_resolver: None,
    };
    let document = Document::parse_with_options(source, options)
        .map_err(|_| FleetBenchmarkError::MalformedXml)?;
    if document.descendants().any(|node| node.is_pi()) {
        return Err(FleetBenchmarkError::ForbiddenDeclaration);
    }

    let root = document.root_element();
    expect_element(root, "fleet-benchmark", ROOT_ATTRIBUTES)?;
    if required_attribute(root, "version")? != "1" {
        return Err(FleetBenchmarkError::UnsupportedVersion);
    }

    let mut children = root.children().filter(Node::is_element);
    let couriers = children
        .next()
        .ok_or(FleetBenchmarkError::InvalidStructure)?;
    if children.next().is_some() {
        return Err(FleetBenchmarkError::InvalidStructure);
    }

    let id = required_attribute(root, "id")?;
    if id.len() > MAX_IDENTIFIER_BYTES
        || !id.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(FleetBenchmarkError::InvalidIdentifier);
    }

    let city = required_attribute(root, "city")?;
    if city.trim().is_empty()
        || city != city.trim()
        || city.chars().count() > MAX_CITY_CHARS
        || city.chars().any(char::is_control)
    {
        return Err(FleetBenchmarkError::InvalidCity);
    }

    Ok(FleetBenchmark {
        id: id.to_owned(),
        city: city.to_owned(),
        stop_count: bounded_number(root, "stops", 1, MAX_STOPS)?,
        shift_minutes: bounded_number(root, "shift-minutes", MIN_SHIFT_MINUTES, MAX_SHIFT_MINUTES)?,
        seed: number(root, "seed")?,
        couriers: load_couriers(couriers)?,
    })
}

fn load_couriers(node: Node<'_, '_>) -> Result<CourierMix, FleetBenchmarkError> {
    expect_element(node, "couriers", COURIER_ATTRIBUTES)?;
    if node.children().any(|child| child.is_element()) {
        return Err(FleetBenchmarkError::InvalidStructure);
    }

    let mix = CourierMix {
        vehicle: bounded_number(node, "vehicle", 0, MAX_COURIERS)?,
        bicycle: bounded_number(node, "bicycle", 0, MAX_COURIERS)?,
        walking: bounded_number(node, "walking", 0, MAX_COURIERS)?,
    };
    if !(1..=MAX_COURIERS).contains(&(mix.vehicle + mix.bicycle + mix.walking)) {
        return Err(FleetBenchmarkError::OutOfRange("couriers"));
    }
    Ok(mix)
}

fn expect_element(
    node: Node<'_, '_>,
    tag: &str,
    allowed: &[&str],
) -> Result<(), FleetBenchmarkError> {
    if node.tag_name().name() != tag
        || node.tag_name().namespace().is_some()
        || node.namespaces().len() != 0
        || node
            .attributes()
            .any(|attr| attr.namespace().is_some() || !allowed.contains(&attr.name()))
        || node.children().any(|child| {
            child.is_text() && !child.text().is_some_and(|value| value.trim().is_empty())
        })
    {
        return Err(FleetBenchmarkError::InvalidStructure);
    }
    Ok(())
}

fn required_attribute<'a>(
    node: Node<'a, '_>,
    name: &'static str,
) -> Result<&'a str, FleetBenchmarkError> {
    node.attribute(name)
        .ok_or(FleetBenchmarkError::MissingAttribute(name))
}

fn number<T: std::str::FromStr>(
    node: Node<'_, '_>,
    name: &'static str,
) -> Result<T, FleetBenchmarkError> {
    let value = required_attribute(node, name)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(FleetBenchmarkError::InvalidNumber(name));
    }
    value
        .parse()
        .map_err(|_| FleetBenchmarkError::InvalidNumber(name))
}

fn bounded_number(
    node: Node<'_, '_>,
    name: &'static str,
    min: u32,
    max: u32,
) -> Result<u32, FleetBenchmarkError> {
    let value = number(node, name)?;
    if !(min..=max).contains(&value) {
        return Err(FleetBenchmarkError::OutOfRange(name));
    }
    Ok(value)
}
