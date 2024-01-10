use crate::parse_js::utility::{get_definition, run_rustfmt};
use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};

pub struct RustClass {
    path: String,
    name: String,
    constructor: Option<RustMethod>,
    methods: Vec<RustMethod>,
}

impl RustClass {
    pub fn new(name: &str) -> RustClass {
        RustClass {
            path: format!("rustominion/src/rustominion/generated/{}.rs", name.to_lowercase()),
            name: name.to_owned(),
            constructor: None,
            methods: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn filename(&self) -> String {
        self.name.to_lowercase().clone()
    }

    pub fn generate(&mut self, content: &str, version: &str) -> Result<(), Box<dyn Error>> {
        // Check version at top of file. If version exists and is same as current, don't do anything to the file
        if self.is_existing_file_ok(version) {
            return Ok(());
        }

        let mut writer = LineWriter::new(File::create(&self.path)?);
        // write metadata
        writeln!(writer, "// version: {version}")?;

        // TODO: Only write readonly if there are attributes, we need to check this
        writeln!(writer, "#[readonly::make]")?;
        writeln!(writer, "#[allow(dead_code)]")?;
        writeln!(writer, "struct {} {{", self.name)?;
        // Identify constructor args
        static RE_CONSTRUCTOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"function (\w+)\((.*?)\) \{").unwrap());
        let capture = RE_CONSTRUCTOR.captures(content).unwrap();
        let (constructor_start, [matched_name, constructor_args]) = capture.extract();
        let constructor_args = constructor_args
            .split(',')
            .map(|s| s.trim().to_case(Case::Snake))
            .collect::<Vec<String>>();
        assert_eq!(self.name, matched_name);
        let mut index = capture.get(0).unwrap().end();
        // Identify attributes
        static RE_ATTRIBUTES: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:this|self)\.(\w+) = (.*?);\n").unwrap());
        let mut last_index = 0;
        // Identify start of methods
        static RE_METHODS: Lazy<Regex> = Lazy::new(|| Regex::new(r" +(?:this|self)\.(\w+) = function \(\) \{").unwrap());
        let first_method_at = RE_METHODS.find_at(content, index);
        let index_of_first_method = match first_method_at {
            Some(m) => m.start(),
            None => {
                let constructor = get_definition(constructor_start, content).unwrap();
                let i = capture.get(0).unwrap().start();
                constructor.len() + i
            }
        };
        let args_assignment = &content[index..index_of_first_method].trim();

        let mut class_attributes = Vec::new();

        for capture in RE_ATTRIBUTES.captures_iter(&content[index..]) {
            let end = capture.get(0).unwrap().end();
            if index + end >= index_of_first_method {
                break;
            }
            last_index = end;
            let (_, [attr, arg]) = capture.extract();
            let snake_attr = attr.to_case(Case::Snake);
            writeln!(writer, "    {}: u8, // TODO: initialized in constructor from {}", snake_attr, arg)?;
            class_attributes.push(snake_attr);
        }

        writeln!(writer, "}}\n\n#[allow(dead_code)]\nimpl {} {{", self.name)?;

        // write constructor
        writeln!(
            writer,
            "    pub fn new({}) -> Self {{",
            constructor_args
                .iter()
                .map(|a| format!("{}: u8", a))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(writer, "        Self {{")?;
        for attribute in class_attributes {
            if constructor_args.contains(&attribute) {
                writeln!(writer, "            {},", attribute)?;
            } else {
                writeln!(writer, "            {}: 0,", attribute)?;
            }
        }
        writeln!(writer, "        }}")?;
        writeln!(writer, "        // TODO: ")?;
        for line in args_assignment.split('\n') {
            writeln!(writer, "        //{line}")?;
        }
        writeln!(writer, "    }}")?;

        // write methods
        let method_slice = &content[index_of_first_method..];
        let mut last_index = 0;
        for capture in RE_METHODS.captures_iter(method_slice) {
            let whole_match = capture.get(0).unwrap();
            let name = capture.get(1).unwrap().as_str();
            let def = get_definition(whole_match.as_str(), &method_slice[whole_match.start()..]).unwrap();
            writeln!(
                writer,
                "    pub fn {}(&self) {{\n        todo!();\n        // TODO:",
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

    fn is_existing_file_ok(&self, version: &str) -> bool {
        // Open the file
        let file = match File::open(&self.path) {
            Ok(file) => file,
            _ => return false,
        };

        // Read the file line by line
        let reader = BufReader::new(file);
        let mut first_line = String::new();

        // Read the first line
        let mut lines = reader.lines();
        if let Some(line) = lines.next() {
            match line {
                Ok(l) => first_line = l,
                _ => return false,
            }
        } else {
            // File is empty or cannot read the first line
            return false;
        }

        // Check if the first line contains the correct version
        if !first_line.trim().eq(format!("// version: {version}").as_str()) {
            return false;
        }

        // User still has to rework the file if it contains any _TODO
        for line in lines {
            match line {
                Ok(content) => {
                    if content.contains("TODO") {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }
}

impl Drop for RustClass {
    fn drop(&mut self) {
        run_rustfmt(self.path.as_str()).expect("Could not run rustfmt");
    }
}

struct RustArgument {
    name: String,
    is_option: bool,
}

struct RustMethod {
    name: String,
    args: Vec<RustArgument>,
    body: Vec<String>,
}
