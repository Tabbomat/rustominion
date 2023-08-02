#[path = "src/bin/parse_js/deserialize.rs"]
mod deserialize;
#[path = "src/bin/parse_js/gen_class.rs"]
mod gen_class;
#[path = "src/bin/parse_js/gen_enum_json.rs"]
mod gen_enum_json;
#[path = "src/bin/parse_js/generate.rs"]
mod generate;
#[path = "src/bin/parse_js/utility.rs"]
mod utility;

fn main() {
    println!("cargo:rerun-if-changed=data");
    // TODO: Enable this when parse_js is finished
    // deserialize::unpack_map_js().expect("TODO: panic message");
}
