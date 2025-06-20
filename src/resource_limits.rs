use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct ResourceLimits {
    /// The largest size (in bytes) core file that may be created.
    core: Option<u32>,

    /// The maximum amount of cpu time (in seconds) to be used by each
    /// process.
    #[serde(rename = "CPU")]
    cpu: Option<u32>,

    /// The maximum size (in bytes) of the data segment for a process; this
    /// defines how far a program may extend its break with the sbrk(2)
    /// system call.
    data: Option<u32>,

    /// The largest size (in bytes) file that may be created.
    file_size: Option<u32>,

    /// The maximum size (in bytes) which a process may lock into memory
    /// using the mlock(2) function.
    memory_lock: Option<u32>,

    /// The maximum number of open files for this process.  Setting this
    /// value in a system wide daemon will set the sysctl(3) kern.maxfiles
    /// (SoftResourceLimits) or kern.maxfilesperproc (HardResourceLimits)
    /// value in addition to the setrlimit(2) values.
    number_of_files: Option<u32>,

    /// The maximum number of simultaneous processes for this UID. Setting
    /// this value in a system wide daemon will set the sysctl(3)
    /// kern.maxproc (SoftResourceLimits) or kern.maxprocperuid
    /// (HardResourceLimits) value in addition to the setrlimit(2) values.
    number_of_processes: Option<u32>,

    /// The maximum size (in bytes) to which a process's resident set size
    /// may grow.  This imposes a limit on the amount of physical memory to
    /// be given to a process; if memory is tight, the system will prefer
    /// to take memory from processes that are exceeding their declared
    /// resident set size.
    resident_set_size: Option<u32>,

    /// The maximum size (in bytes) of the stack segment for a process;
    /// this defines how far a program's stack segment may be extended.
    /// Stack extension is performed automatically by the system.
    stack: Option<u32>,
}
