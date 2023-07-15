use crate::deserialize::{JavascriptContentType, JavascriptDefinition};
use crate::gen_class::RustClass;
use crate::gen_enum_json::RustEnumJson;
use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct RustCodeGenerator {
    classes: Vec<RustClass>,
    enum_jsons: Vec<RustEnumJson>,
    version: String,
}

impl RustCodeGenerator {
    pub fn new(version: String) -> Result<RustCodeGenerator, Box<dyn Error>> {
        fs::create_dir_all("src/rustominion/generated")?;
        fs::create_dir_all("src/bin/generate_json_data/generated")?;

        Ok(RustCodeGenerator {
            classes: vec![],
            enum_jsons: vec![],
            version,
        })
    }

    pub fn generate(&mut self, definition: &JavascriptDefinition, content: &str) -> Result<(), Box<dyn Error>> {
        match definition.type_ {
            JavascriptContentType::Class => self.generate_class(&definition.start, content),
            JavascriptContentType::MapEnumToStatic => self.generate_enum_json(&definition.start, content),
        }
    }

    fn generate_class(&mut self, class_start: &str, content: &str) -> Result<(), Box<dyn Error>> {
        static RE_CLASSNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"var (\w+) = function").unwrap());
        let classname = RE_CLASSNAME.captures(class_start).unwrap().get(1).unwrap().as_str();
        let mut class = RustClass::new(classname);
        class.generate(content, self.version.as_str())?;
        self.classes.push(class);

        Ok(())
    }

    fn generate_enum_json(&self, map_start: &str, content: &str) -> Result<(), Box<dyn Error>> {
        return Ok(());
        todo!()
    }
}

impl Drop for RustCodeGenerator {
    fn drop(&mut self) {
        let mut class_mod = BufWriter::new(File::create("src/rustominion/generated/mod.rs").expect("Could not open mod.rs"));
        for class in &self.classes {
            writeln!(class_mod, "pub mod {};", class.filename()).expect("Could not write to mod.rs");
        }
    }
}
