use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ptr::write;

pub struct RustClass {
    writer: BufWriter<File>,
    name: String,
}

impl RustClass {
    pub fn new(name: &str) -> Result<RustClass, Box<dyn Error>> {
        let filename = format!("src/rustominion/generated/{}.rs", name.to_lowercase());
        Ok(RustClass {
            writer: BufWriter::new(File::create(filename)?),
            name: name.to_owned(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn filename(&self) -> String {
        self.name.to_lowercase().to_owned()
    }

    pub fn generate(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        // Workflow is "generate class.rs" -> User checks it and indicates it in the file
        // on subsequent runs: If content is the same as during last "generate class.rs", check if user indicated it as finished
        // if both yes: don't touch the file, shortcut return early

        // TODO: Only write readonly if there are attributes
        // writeln!(self.writer, "#[readonly::make]")?;
        writeln!(self.writer, "struct {} {{", self.name)?;
        // TODO: write attributes
        writeln!(self.writer, "}}\n\nimpl {} {{", self.name)?;
        // TODO: write methods
        writeln!(self.writer, "}}\n")?;

        // write metadata
        // TODO: finished as attribute of RustClass
        writeln!(self.writer, "// finished: false")?;
        writeln!(self.writer, "// old source: TODO")?;

        for line in content.split('\n') {
            writeln!(self.writer, "// {}", line)?;
        }

        Ok(())
    }
}
