use crate::BuiltinRecipe;

macro_rules! example {
    ($id:literal) => {
        example!($id, "")
    };
    ($id:literal, $variant:literal) => {{
        const EXAMPLE_PATH: &str = concat!("recipes/examples/", $id, "/", $variant, "problem.json");
        BuiltinRecipe {
            id: $id,
            xml: include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../recipes/",
                $id,
                ".xml"
            )),
            problem_json: include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../recipes/examples/",
                $id,
                "/",
                $variant,
                "problem.json"
            )),
            example_path: EXAMPLE_PATH,
        }
    }};
}

static RECIPES: &[BuiltinRecipe] = &[
    example!("airline-crew-assignment"),
    example!("airport-gate-assignment"),
    example!("call-center-staffing"),
    example!("cluster-workload-placement"),
    example!("dental-clinic-room-allocation"),
    example!("field-service"),
    example!("fleet-pickup-delivery"),
    example!("healthcare-supply-delivery"),
    example!("high-school-timetabling"),
    example!("hospital-nonclinical-shift-coverage"),
    example!("loading-dock-assignment"),
    example!("operating-room-block-allocation"),
    example!("retail-staffing-what-if", "baseline/"),
];

pub fn builtin_recipes() -> &'static [BuiltinRecipe] {
    RECIPES
}

pub fn builtin_recipe(id: &str) -> Option<&'static BuiltinRecipe> {
    RECIPES.iter().find(|recipe| recipe.id() == id)
}
