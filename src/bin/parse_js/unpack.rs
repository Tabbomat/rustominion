use serde::Deserialize;
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

#[inline(never)]
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
        (Some(v), Some(f)) => Some((v, format!("data/{}", f))),
        _ => None
    }
}

#[inline(never)]
pub fn unpack_map_js() -> Result<(), Box<dyn Error>> {
    let w = find_latest_map();

    let file = File::open(w.unwrap().1)?;
    let reader = BufReader::new(file);

    let m: JavascriptMap = serde_json::from_reader(reader)?;

    assert_eq!(m.sources.len(), m.sources_content.len());

    Ok(())
}

