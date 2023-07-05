use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavascriptMap {
    version: u32,
    sources: Vec<String>,
    sources_content: Vec<String>
}

pub fn unpack_map_js() -> Result<JavascriptMap, Box<dyn Error>> {
    let file = File::open("data/dominion-webclient-body-1.7.1.6.map.js")?;
    let reader = BufReader::new(file);

    let m = serde_json::from_reader(reader)?;

    Ok(m)
}
