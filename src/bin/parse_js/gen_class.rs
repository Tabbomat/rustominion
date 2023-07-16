use crate::utility::{get_definition, run_rustfmt};
use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct RustClass {
    path: String,
    name: String,
}

impl RustClass {
    pub fn new(name: &str) -> RustClass {
        RustClass {
            path: format!("src/rustominion/generated/{}.rs", name.to_lowercase()),
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn filename(&self) -> String {
        self.name.to_lowercase().clone()
    }

    pub fn generate(&mut self, content: &str, version: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Check version at top of file. If version exists and is same as current, don't do anything to the file
        let mut writer = BufWriter::new(File::create(&self.path)?);
        // write metadata
        writeln!(writer, "// version: {version}")?;

        // TODO: Only write readonly if there are attributes, we need to check this
        writeln!(writer, "#[readonly::make]")?;
        writeln!(writer, "#[allow(dead_code)]")?;
        writeln!(writer, "struct {} {{", self.name)?;
        // Identify constructor args
        static RE_CONSTRUCTOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"function (\w+)\((.*?)\) \{").unwrap());
        let capture = RE_CONSTRUCTOR.captures(content).unwrap();
        let (_, [matched_name, constructor_args]) = capture.extract();
        assert_eq!(self.name, matched_name);
        let mut index = capture.get(0).unwrap().end();
        // Identify attributes
        static RE_ATTRIBUTES: Lazy<Regex> = Lazy::new(|| Regex::new(r"this\.(\w+) = (.*?);\n").unwrap());
        let mut last_index = 0;
        // Identify start of methods
        static RE_METHODS: Lazy<Regex> = Lazy::new(|| Regex::new(r" +this\.(\w+) = function \(\) \{").unwrap());
        let index_of_first_method = RE_METHODS.find_at(content, index).unwrap().start();

        for capture in RE_ATTRIBUTES.captures_iter(&content[index..]) {
            let end = capture.get(0).unwrap().end();
            if index + end >= index_of_first_method {
                break;
            }
            last_index = end;
            let (_, [attr, arg]) = capture.extract();
            writeln!(
                writer,
                "    {}: u8, // CHECK THIS: from constructor arg {}",
                attr.to_case(Case::Snake),
                arg
            )?;
        }

        writeln!(writer, "}}\n\n#[allow(dead_code)]\nimpl {} {{", self.name)?;
        let method_slice = &content[index_of_first_method..];
        for capture in RE_METHODS.captures_iter(method_slice) {
            let whole_match = capture.get(0).unwrap();
            let name = capture.get(1).unwrap().as_str();
            let def = get_definition(whole_match.as_str(), &method_slice[whole_match.start()..]).unwrap();
            writeln!(
                writer,
                "    pub fn {}(&self) {{\n        todo!();\n        // CHECK THIS:",
                name.to_case(Case::Snake)
            )?;
            for line in def.split('\n') {
                writeln!(writer, "        //{line}")?;
            }
            writeln!(writer, "    }}")?;
            last_index = whole_match.start() + def.len() + 1;
        }
        writeln!(writer, "}}\n")?;
        index = index_of_first_method + last_index;

        for line in content[index..].split('\n') {
            writeln!(writer, "// {line}")?;
        }

        // TODO: use _createClass

        Ok(())
    }
}

impl Drop for RustClass {
    fn drop(&mut self) {
        run_rustfmt(self.path.as_str()).expect("Could not run rustfmt");
    }
}