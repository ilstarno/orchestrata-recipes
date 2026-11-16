use orchestrata_recipes::{FieldKind, RecipeError, load_recipe};

const EXAMPLE: &str = include_str!("../recipes/field-service.xml");

#[test]
fn reads_the_field_service_example() {
    let recipe = load_recipe(EXAMPLE).unwrap();
    assert_eq!(recipe.id(), "field-service");
    assert!(recipe.title().contains("technicians"));
    assert!(recipe.scenario().contains("Lift repair"));
    assert_eq!(recipe.inputs().len(), 4);
    assert_eq!(recipe.inputs()[0].kind, FieldKind::StringList);
    assert_eq!(recipe.inputs()[2].kind, FieldKind::IntegerList);
    assert!(!recipe.inputs()[3].required);
    assert_eq!(recipe.outputs()[0].name, "assigned-technicians");
    assert!(recipe.outputs()[0].description.contains("input order"));
}

#[test]
fn rejects_unsupported_versions_and_invalid_identifiers() {
    let wrong_version = EXAMPLE.replace("recipe version=\"1\"", "recipe version=\"2\"");
    assert_eq!(
        load_recipe(&wrong_version),
        Err(RecipeError::UnsupportedVersion)
    );
    let invalid_id = EXAMPLE.replace("id=\"field-service\"", "id=\"Unexpected/Path\"");
    assert_eq!(
        load_recipe(&invalid_id),
        Err(RecipeError::InvalidIdentifier)
    );
}

#[test]
fn rejects_unknown_tags_attributes_and_namespaces() {
    for xml in [
        EXAMPLE.replace("<inputs>", "<unexpected>"),
        EXAMPLE.replace(" id=", " extra=\"true\" id="),
        EXAMPLE.replace("<recipe ", "<recipe xmlns:r=\"urn:reserved\" "),
        EXAMPLE.replace("<outputs>", "<outputs>untyped data"),
    ] {
        assert!(load_recipe(&xml).is_err());
    }
}

#[test]
fn rejects_duplicate_fields_and_non_allowlisted_flags_or_types() {
    let duplicate_name = EXAMPLE.replace("technician-names", "job-names");
    assert_eq!(
        load_recipe(&duplicate_name),
        Err(RecipeError::DuplicateField)
    );
    let unknown_type = EXAMPLE.replace("string-list", "object");
    assert_eq!(
        load_recipe(&unknown_type),
        Err(RecipeError::InvalidFieldKind)
    );
    let unknown_flag = EXAMPLE.replace("required=\"true\"", "required=\"yes\"");
    assert_eq!(
        load_recipe(&unknown_flag),
        Err(RecipeError::InvalidRequiredFlag)
    );
}

#[test]
fn declarations_beyond_the_xml_prolog_are_forbidden() {
    let empty_dtd = EXAMPLE.replace("<recipe ", "<!DOCTYPE recipe []><recipe ");
    assert_eq!(
        load_recipe(&empty_dtd),
        Err(RecipeError::ForbiddenDeclaration)
    );
    let processing_instruction = EXAMPLE.replace("<recipe ", "<?unsupported ignored?><recipe ");
    assert_eq!(
        load_recipe(&processing_instruction),
        Err(RecipeError::ForbiddenDeclaration)
    );
}

#[test]
fn bounds_document_bytes_and_list_sizes() {
    let too_large = " ".repeat(32_769);
    assert_eq!(load_recipe(&too_large), Err(RecipeError::TooLarge));

    let fields: String = (0..33)
        .map(|idx| {
            format!(
                "<field name=\"field-{idx}\" type=\"integer\" required=\"true\"><description>Integer.</description></field>"
            )
        })
        .collect();
    let first = EXAMPLE.find("<inputs>").unwrap() + "<inputs>".len();
    let end = EXAMPLE.find("</inputs>").unwrap();
    let too_many_fields = format!("{}{fields}{}", &EXAMPLE[..first], &EXAMPLE[end..]);
    assert_eq!(
        load_recipe(&too_many_fields),
        Err(RecipeError::InvalidFieldCount)
    );
}

#[test]
fn missing_sections_and_empty_field_descriptions_are_not_defaults() {
    let no_description = EXAMPLE.replace(
        "<description>For a feasible plan, one technician index per job in input order. The example preference yields 0,1,2.</description>",
        "<description/>",
    );
    assert_eq!(load_recipe(&no_description), Err(RecipeError::InvalidText));
    let extra_section = EXAMPLE.replace("<inputs>", "<title>A second title</title><inputs>");
    assert_eq!(
        load_recipe(&extra_section),
        Err(RecipeError::InvalidStructure)
    );
}

#[test]
fn loads_hiring_operations_recipes_as_version_one_contracts() {
    for (id, xml) in [
        (
            "hiring-interview-scheduling",
            include_str!("../recipes/hiring-interview-scheduling.xml"),
        ),
        (
            "hiring-interviewer-recovery",
            include_str!("../recipes/hiring-interviewer-recovery.xml"),
        ),
        (
            "hiring-coordinator-checkpoints",
            include_str!("../recipes/hiring-coordinator-checkpoints.xml"),
        ),
    ] {
        let recipe = load_recipe(xml).unwrap_or_else(|err| panic!("{id}: {err:?}"));
        assert_eq!(recipe.id(), id);
        assert!(!recipe.inputs().is_empty());
        assert!(!recipe.outputs().is_empty());
    }
}

#[test]
fn loads_traffic_planning_recipes_as_version_one_contracts() {
    for (id, xml) in [
        (
            "dns-regional-steering",
            include_str!("../recipes/dns-regional-steering.xml"),
        ),
        (
            "backend-capacity-placement",
            include_str!("../recipes/backend-capacity-placement.xml"),
        ),
        (
            "canary-drain-failback",
            include_str!("../recipes/canary-drain-failback.xml"),
        ),
    ] {
        let recipe = load_recipe(xml).unwrap_or_else(|err| panic!("{id}: {err:?}"));
        assert_eq!(recipe.id(), id);
        assert!(!recipe.inputs().is_empty());
        assert!(!recipe.outputs().is_empty());
    }
}

#[test]
fn loads_financial_operations_recipes_as_version_one_contracts() {
    for (id, xml) in [
        (
            "payment-operations-review-routing",
            include_str!("../recipes/payment-operations-review-routing.xml"),
        ),
        (
            "settlement-reconciliation-backlog",
            include_str!("../recipes/settlement-reconciliation-backlog.xml"),
        ),
        (
            "claims-operations-work-dispatch",
            include_str!("../recipes/claims-operations-work-dispatch.xml"),
        ),
        (
            "adjuster-appointment-recovery",
            include_str!("../recipes/adjuster-appointment-recovery.xml"),
        ),
    ] {
        let recipe = load_recipe(xml).unwrap_or_else(|err| panic!("{id}: {err:?}"));
        assert_eq!(recipe.id(), id);
        assert!(!recipe.inputs().is_empty());
        assert!(!recipe.outputs().is_empty());
    }
}
