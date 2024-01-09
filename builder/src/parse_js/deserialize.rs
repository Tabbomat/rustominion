use crate::parse_js::generate::RustCodeGenerator;
use crate::parse_js::utility::get_definition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavascriptMap {
    sources: Vec<String>,
    sources_content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[readonly::make]
pub struct JavascriptDefinition {
    pub start: String,
    pub type_: JavascriptContentType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum JavascriptContentType {
    Class,
    MapEnumToStatic,
    Function,
}

type JavascriptFileContents = HashMap<String, Vec<JavascriptDefinition>>;

fn find_latest_map() -> Option<(String, String)> {
    // TODO: Download from internet
    let directory = "data";
    let files = fs::read_dir(directory).ok()?;
    let mut latest_version: Option<String> = None;
    let mut latest_file: Option<String> = None;

    for file in files.flatten() {
        if let Some(file_name) = file.file_name().to_str() {
            if file_name.ends_with(".map.js") {
                let version = file_name.trim_end_matches(".map.js").rsplit('-').next()?.to_owned();

                if latest_version.is_none() || version > latest_version.clone().unwrap() {
                    latest_version = Some(version);
                    latest_file = Some(file_name.to_owned());
                }
            }
        }
    }

    match (latest_version, latest_file) {
        (Some(v), Some(f)) => Some((v, format!("data/{f}"))),
        _ => None,
    }
}

/// General idea: Iterate twice over all data in the .map.js. Keep hashes of all successfully converted data
/// First run: Identify all classes and enums. If current hash != old hash, generate raw definitions
/// and ask user to verify and compile/run again. Only one class / enum at a time
/// If first run finished with all hashes equal to old ones, do a second run:
/// We now have compiled all structs and enums, we now can initialize structs and serialize them.
pub fn unpack_map_js() -> Result<(), Box<dyn Error>> {
    let Some((version, path)) = find_latest_map() else {
        panic!("Could not find map.")
    };

    let javascript_map: JavascriptMap = serde_json::from_reader(BufReader::new(File::open(path)?))?;
    let javascript_contents: JavascriptFileContents = serde_json::from_reader(BufReader::new(File::open("../../../data/map_contents.json")?))?;

    assert_eq!(javascript_map.sources.len(), javascript_map.sources_content.len());

    let mut generator = RustCodeGenerator::new(version)?;

    for index in 0..javascript_map.sources.len() {
        let source = &javascript_map.sources[index];
        let content = &javascript_map.sources_content[index];

        if let Some(data) = javascript_contents.get(source) {
            for def in data {
                let content = get_definition(def.start.as_ref(), content).unwrap();
                generator.generate(def, content)?;
            }
        }
    }

    Ok(())
}
