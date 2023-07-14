use crate::utility;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use regex::Regex;
use {once_cell::sync::Lazy};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JavascriptMap {
    sources: Vec<String>,
    sources_content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct JavascriptDefinition {
    start: String,
    type_: JavascriptContentType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum JavascriptContentType {
    Class,
    MapEnumToStatic,
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
                let version = file_name
                    .trim_end_matches(".map.js")
                    .rsplit('-')
                    .next()?
                    .to_owned();

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

#[inline(never)]
/// General idea: Iterate twice over all data in the .map.js. Keep hashes of all successfully converted data
/// First run: Identify all classes and enums. If current hash != old hash, generate raw definitions
/// and ask user to verify and compile/run again. Only one class / enum at a time
/// If first run finished with all hashes equal to old ones, do a second run:
/// We now have compiled all structs and enums, we now can initialize structs and serialize them.
pub fn unpack_map_js() -> Result<(), Box<dyn Error>> {
    let w = find_latest_map();

    let javascript_map: JavascriptMap =
        serde_json::from_reader(BufReader::new(File::open(w.unwrap().1)?))?;
    let javascript_contents: JavascriptFileContents =
        serde_json::from_reader(BufReader::new(File::open("data/map_contents.json")?))?;

    assert_eq!(
        javascript_map.sources.len(),
        javascript_map.sources_content.len()
    );

    for index in 0..javascript_map.sources.len() {
        let source = &javascript_map.sources[index];
        let content = &javascript_map.sources_content[index];

        if let Some(data) = javascript_contents.get(source) {
            for def in data {
                match def.type_ {
                    JavascriptContentType::Class => handle_class_definition(&def.start, content),
                    JavascriptContentType::MapEnumToStatic => {
                        handle_map_enum_static(&def.start, content)
                    }
                }?;
            }
        }
    }

    Ok(())
}

fn handle_class_definition(class_start: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("src/rustominion/generated")?;
    fs::create_dir_all("src/bin/generate_json_data/generated")?;
    // TODO: determine file name
    // TODO: generate mod.rs in /generated, and append file name
    // TODO: generate file content
    // Workflow is "generate class.rs" -> User checks it and indicates it in the file
    // on subsequent runs: If content is the same as during last "generate class.rs", check if user indicated it as finished
    // if both yes: don't touch the file, shortcut return early
    // "var CardName = function () {"
    static RE_CLASSNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"var (\w+) = function").unwrap());
    let classname = RE_CLASSNAME.captures(class_start).unwrap().get(1).unwrap().as_str();
    let filename = format!("src/rustominion/generated/{}.rs", classname.to_lowercase());
    let _definition = utility::get_class_definition(class_start, content).unwrap();
    let mut writer = BufWriter::new(File::create(filename)?);
    writer.write(b"use std::collections::HashMap;\n\n#[readonly::make]")?;
    let mut mod_writer = BufWriter::new(File::create("src/rustominion/generated/mod.rs")?);
    // mod_writer.write()
    todo!()
}

fn handle_map_enum_static(map_start: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let _definition = utility::get_class_definition(map_start, content).unwrap();
    todo!()
}
