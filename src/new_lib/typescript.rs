use std::fs;
use std::path;

use crate::error::{Err, Result};
use crate::new_lib::TemplateGenerator;

pub struct Template;

impl TemplateGenerator for Template {
    fn generate(lib_name: &str, output_dir: path::PathBuf) -> Result<()> {
        let gen_dir = output_dir.join(lib_name);
        if !&gen_dir.is_dir() {
            fs::create_dir_all(&gen_dir).map_err(Err::IOError)?;
        }

        let lib_file = output_dir.join(lib_name).join("index.ts");
        fs::File::create(lib_file).map_err(Err::IOError)?;

        Ok(())
    }
}
