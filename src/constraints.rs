use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Soft and/or hard resource limits to be imposed on a job.
#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct ResourceLimits {
    /// The largest size (in bytes) core file that may be created.
    pub core: Option<u32>,

    /// The maximum amount of cpu time (in seconds) to be used by each
    /// process.
    #[serde(rename = "CPU")]
    pub cpu: Option<u32>,

    /// The maximum size (in bytes) of the data segment for a process.
    ///
    /// This defines how far a program may extend its break with the `sbrk(2)`
    /// system call.
    pub data: Option<u32>,

    /// The largest size (in bytes) file that may be created.
    pub file_size: Option<u32>,

    /// The maximum size (in bytes) which a process may lock into memory
    /// using the `mlock(2)` function.
    pub memory_lock: Option<u32>,

    /// The maximum number of open files for this process.
    ///
    /// Setting this value in a system wide daemon will set the `sysctl(3)`
    /// `kern.maxfiles`
    /// ([`soft_resource_limits`](crate::LaunchAgent::soft_resource_limits))
    /// or `kern.maxfilesperproc`
    /// ([`hard_resource_limits`](crate::LaunchAgent::hard_resource_limits))
    /// value in addition to the `setrlimit(2)` values.
    pub number_of_files: Option<u32>,

    /// The maximum number of simultaneous processes for this UID.
    ///
    /// Setting this value in a system wide daemon will set the `sysctl(3)`
    /// `kern.maxproc`
    /// ([`soft_resource_limits`](crate::LaunchAgent::soft_resource_limits))
    /// or `kern.maxprocperuid`
    /// ([`hard_resource_limits`](crate::LaunchAgent::hard_resource_limits))
    /// value in addition to the `setrlimit(2)` values.
    pub number_of_processes: Option<u32>,

    /// The maximum size (in bytes) to which a process's resident set size
    /// may grow.
    ///
    /// This imposes a limit on the amount of physical memory to be given to a
    /// process; if memory is tight, the system will prefer to take memory from
    /// processes that are exceeding their declared resident set size.
    pub resident_set_size: Option<u32>,

    /// The maximum size (in bytes) of the stack segment for a process.
    ///
    /// This defines how far a program's stack segment may be extended.
    /// Stack extension is performed automatically by the system.
    pub stack: Option<u32>,
}

/// The type of session a job may be run in.
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SessionType {
    Single(String),
    Many(Vec<String>),
}

/// The intended purpose of a job.
#[derive(Clone, Deserialize, Serialize)]
pub enum ProcessType {
    /// Background jobs are generally processes that do work that was not
    /// directly requested by the user.
    ///
    /// The resource limits applied to Background jobs are intended to prevent
    /// them from disrupting the user experience.
    Background,

    /// Standard jobs are equivalent to no
    /// [`process_type`](crate::LaunchAgent::process_type) being
    /// set.
    Standard,

    /// Adaptive jobs move between the [`Background`](ProcessType::Background)
    /// and [`Interactive`](ProcessType::Interactive) classifications based on
    /// activity over XPC connections. See `xpc_transaction_begin(3)` for
    /// details.
    Adaptive,

    /// Interactive jobs run with the same resource limitations as apps,
    /// that is to say, none.
    ///
    /// Interactive jobs are critical to maintaining a responsive user
    /// experience, and this type should only be used if an app's ability to be
    /// responsive depends on it, and cannot be made
    /// [`Adaptive`](ProcessType::Adaptive).
    Interactive,
}
