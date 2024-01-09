mod deserialize;
mod gen_class;
mod gen_enum_json;
mod generate;
mod utility;

fn main() {
    deserialize::unpack_map_js().expect("TODO: panic message");
}
