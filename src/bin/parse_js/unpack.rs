use crate::utility;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JavascriptMap {
    sources: Vec<String>,
    sources_content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ClassHashes {
    card_name: String,
}

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

pub fn unpack_map_js() -> Result<(), Box<dyn Error>> {
    let w = find_latest_map();

    let file = File::open(w.unwrap().1)?;
    let reader = BufReader::new(file);

    let m: JavascriptMap = serde_json::from_reader(reader)?;

    assert_eq!(m.sources.len(), m.sources_content.len());

    for index in 0..m.sources.len() {
        let source = &m.sources[index];
        let content = &m.sources_content[index];

        if source.ends_with("cards/card-names.js") {
            const START_CARDNAME: &str = "var CardName = function () {";
            let card_name = utility::get_class_definition(START_CARDNAME, content).unwrap();
            println!("{}", &card_name);
        }
    }

    Ok(())
}
