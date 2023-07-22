use std::fs;
use std::path;

use crate::error::{Err, Result};
use crate::new_lib::TemplateGenerator;

pub struct Template;

impl TemplateGenerator for Template {
    fn generate(lib_name: &str, output_dir: path::PathBuf) -> Result<()> {
        let gen_dir = output_dir.join(lib_name).join("src").join("generated");
        if !&gen_dir.is_dir() {
            fs::create_dir_all(&gen_dir).map_err(Err::IOError)?;
        }
        let mod_file = gen_dir.join("mod.rs");
        fs::File::create(mod_file).map_err(Err::IOError)?;

        let lib_file = output_dir.join(lib_name).join("src").join("lib.rs");
        let lib_content = r#"mod generated;

pub use generated::*;
"#;
        fs::write(lib_file, lib_content).map_err(Err::IOError)?;

        let manifest_path = output_dir.join(lib_name).join("Cargo.toml");
        let mainfest_content = r#"[package]
name = "types"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bincode = { version = "=2.0.0-rc.3" }
bincode_macro = "0.1"
"#;
        let mainfest_content = str::replace(mainfest_content, "types", lib_name);
        fs::write(manifest_path, mainfest_content).map_err(Err::IOError)?;

        Ok(())
    }
}
