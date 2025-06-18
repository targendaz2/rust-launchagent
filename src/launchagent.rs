use anyhow::{Context, Result};
use derive_builder::Builder;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Builder, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(default, setter(into, strip_option))]
pub struct LaunchAgent {
    label: String,
    program: Option<String>,
    #[builder(setter(each(name = "program_argument", into)))]
    program_arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
    standard_in_path: Option<String>,
    standard_out_path: Option<String>,
    standard_error_path: Option<String>,
    working_directory: Option<String>,
    soft_resource_limits: Option<ResourceLimits>,
    hard_resource_limits: Option<ResourceLimits>,
    run_at_load: Option<bool>,
    start_interval: Option<u32>,
    start_calendar_interval: Option<Vec<CalendarInterval>>,
    start_on_mount: Option<bool>,
    watch_paths: Option<Vec<String>>,
    queue_directories: Option<Vec<String>>,
    keep_alive: Option<KeepAlive>,
    user_name: Option<String>,
    group_name: Option<String>,
    init_groups: Option<bool>,
    umask: Option<f32>,
    root_directory: Option<String>,
    abandon_process_group: Option<bool>,
    exit_time_out: Option<u32>,
    time_out: Option<u32>,
    throttle_interval: Option<u32>,
    legacy_timers: Option<bool>,
    nice: Option<i8>,
}

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

#[derive(Builder, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
struct ResourceLimits {
    #[serde(rename = "CPU")]
    cpu: Option<u32>,
    file_size: Option<u32>,
    number_of_files: Option<u32>,
    core: Option<u32>,
    data: Option<u32>,
    memory_lock: Option<u32>,
    number_of_processes: Option<u32>,
    resident_set_size: Option<u32>,
    stack: Option<u32>,
}

#[derive(Builder, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
struct CalendarInterval {
    month: Option<u32>,
    day: Option<u32>,
    weekday: Option<u32>,
    hour: Option<u32>,
    minute: Option<u32>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
enum KeepAlive {
    Bool(bool),
    Object {
        successful_exit: Option<bool>,
        crashed: Option<bool>,
        network_state: Option<bool>,
        path_state: Option<HashMap<String, bool>>,
        other_job_enabled: Option<HashMap<String, bool>>,
        after_initial_demand: Option<bool>,
    },
}
