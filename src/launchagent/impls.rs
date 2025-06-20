use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::structs::{LaunchAgent, LaunchAgentBuilder};

impl LaunchAgent {
    pub fn new(label: String, program: &str) -> Self {
        LaunchAgentBuilder::default()
            .label(label)
            .program(program)
            .build()
            .unwrap()
    }

    pub fn new_with_args(label: String, program_arguments: Vec<&str>) -> Self {
        let program_arguments: Vec<String> =
            program_arguments.into_iter().map(String::from).collect();

        LaunchAgentBuilder::default()
            .label(label)
            .program_arguments(program_arguments)
            .build()
            .unwrap()
    }

    pub fn save<P: AsRef<Path>>(&self, out_dir: P) -> Result<()> {
        let path = PathBuf::from(out_dir.as_ref()).join(format!("{}.plist", self.label));

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create parent directories for {parent:?}"))?;
        }

        plist::to_file_xml(&path, &self)
            .with_context(|| format!("Failed to save LaunchAgent to {path:?}"))
    }
}
