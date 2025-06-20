use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::keep_alive::KeepAlive;
use crate::mach_service::MachServiceConfig;
use crate::socket::SocketValue;
use crate::unions::{StringOrF32, StringOrVec};

#[derive(Builder, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(default, setter(into, strip_option))]
pub struct LaunchAgent {
    /// Uniquely identifies the job to `launchd`.
    pub label: String,

    /// Whether the job should be loaded by default.
    ///
    /// This key may be overridden through the `enable` subcommand of
    /// `launchctl(3)`.
    pub disabled: Option<bool>,

    /// The user to run the job as.
    ///
    /// Only applicable for services that are loaded into the privileged system
    /// domain.
    pub user_name: Option<String>,

    /// The group to run the job as.
    ///
    /// Only applicable for services that are loaded into the privileged system
    /// domain. If [`user_name`](Self::user_name) is set and
    /// [`group_name`](Self::group_name) is not, then the group will be set to
    /// the primary group of the user.
    pub group_name: Option<String>,

    /// If not `None`, the daemon expects to be run as if it were launched from
    /// `inetd`.
    /// <div class="warning">For new projects, this key should be avoided.</div>
    pub inetd_compatibility: Option<InetdCompatibility>,

    /// This configuration file only applies to the hosts listed.
    #[deprecated(note = "This key is no longer supported.")]
    limit_load_to_hosts: Option<Vec<String>>,

    /// This configuration file only applies to hosts NOT listed.
    #[deprecated(note = "This key is no longer supported.")]
    limit_load_from_hosts: Option<Vec<String>>,

    /// This configuration file only applies to sessions of the type(s)
    /// specified.
    ///
    /// Only applies to jobs which are agents. There are no
    /// distinct sessions in the privileged system context.
    pub limit_load_to_session_type: Option<SessionType>,

    /// This configuration file only applies to the hardware listed.
    ///
    /// Each key in the dictionary defines a subdomain of the "hw" sysctl(3)
    /// domain. Each value of the key defines valid values for the job to load.
    /// So a key of "model" with an array specifying only "MacBookPro4,2" would
    /// only load on a machine whose "hw.model" value was "MacBookPro4,2".
    pub limit_load_to_hardware: Option<HashMap<String, Vec<String>>>,

    /// This configuration file only applies to the hardware NOT listed.
    ///
    /// Each key in the dictionary defines a subdomain of the "hw"
    /// sysctl(3) domain. Each value of the key defines values where the job will
    /// not load. So a key of "model" with an array specifying only
    /// "MacBookPro4,2" would not load the job on a machine whose "hw.model"
    /// value was "MacBookPro4,2".
    pub limit_load_from_hardware: Option<HashMap<String, Vec<String>>>,

    /// Maps to the first argument of `execv(3)` and indicates the
    /// absolute path to the executable for the job.
    ///
    /// If missing, then the first element of the array of strings provided to
    /// [`program_arguments`](Self::program_arguments) will be used instead.
    /// Required in the absence of
    /// [`program_arguments`](Self::program_arguments) and
    /// [`bundle_program`](Self::bundle_program).
    ///
    /// NOTE: [`program`](Self::program) must be an absolute path. Previous
    /// versions of `launchd` did not enforce this requirement but failed to
    /// run the job.
    pub program: Option<String>,

    /// Maps to the first argument of `execv(3)` and is an app-bundle
    /// relative path to the executable for the job.
    ///
    /// Only supported for plists that are installed using `SMAppService`.
    pub bundle_program: Option<String>,

    /// Maps to the second argument of `execvp(3)` and specifies the
    /// argument vector to be passed to the job when a process is spawned.
    /// Required in the absence of [`program`](Self::program).
    ///
    /// IMPORTANT: Many people are confused by this key. Please read
    /// `execvp(3)` very carefully!
    ///
    /// NOTE: In the absence of [`program`](Self::program), the first
    /// element of [`program_arguments`](Self::program_arguments) may be either
    /// an absolute path, or a relative path which is resolved using
    /// `_PATH_STDPATH`.
    #[builder(setter(each(name = "program_argument", into)))]
    pub program_arguments: Option<Vec<String>>,

    /// Causes `launchd` to use the `glob(3)` mechanism to update the
    /// program arguments before invocation.
    pub enable_globbing: Option<bool>,

    /// Instructs `launchd` that the job uses `xpc_transaction_begin(3)` and
    /// `xpc_transaction_end(3)` to track outstanding transactions.
    ///
    /// When a process has an outstanding transaction, it is considered active,
    /// otherwise inactive. A transaction is automatically created when an XPC
    /// message expecting a reply is received, until the reply is sent or the
    /// request message is discarded. When `launchd` stops an active process,
    /// it sends `SIGTERM` first, and then `SIGKILL` after a reasonable
    /// timeout. If the process is inactive, `SIGKILL` is sent immediately.
    pub enable_transactions: Option<bool>,

    /// Opts the job into the system's Pressured Exit facility.
    ///
    /// Use of this key implies
    /// [`enable_transactions`](Self::enable_transactions) , and also lets the
    /// system consider the process eligible for reclamation under memory
    /// pressure when it's inactive. See `xpc_main(3)` for details. Jobs that
    /// opt into Pressured Exit will be automatically relaunched if they exit
    /// or crash while holding open transactions.
    ///
    /// NOTE: `launchd(8)` does not respect
    /// [`enable_pressured_exit`](Self::enable_pressured_exit) for jobs that
    /// have [`keep_alive`](Self::keep_alive) set to `true`.
    ///
    /// IMPORTANT: Jobs which opt into Pressured Exit will ignore `SIGTERM`
    /// rather than exiting by default, so a `dispatch(3)` source must be used
    /// when handling this signal.
    pub enable_pressured_exit: Option<bool>,

    /// Does nothing if set to `true`. If set to `false`, this is
    /// equivalent to specifying a `true` value for
    /// [`keep_alive`](Self::keep_alive).
    #[deprecated(
        note = "This key should not be used. Please remove this key from your launchd.plist."
    )]
    on_demand: Option<bool>,

    #[deprecated(note = "Please remove this key from your launchd.plist.")]
    #[serde(rename = "ServiceIPC")]
    service_ipc: Option<bool>,

    /// Whether your job is to be kept continuously running or to let demand
    /// and conditions control the invocation.
    ///
    /// The default is `false` and therefore only demand will start the job.
    /// The value may be set to `true` to unconditionally keep the job alive.
    /// Alternatively, a dictionary of conditions may be specified to
    /// selectively control whether `launchd` keeps a job alive or not. If
    /// multiple keys are provided, `launchd` ORs them, thus providing maximum
    /// flexibility to the job to refine the logic and stall if necessary. If
    /// `launchd` finds no reason to restart the job, it falls back on demand
    /// based invocation.  Jobs that exit quickly and frequently when
    /// configured to be kept alive will be throttled to conserve system
    /// resources. The use of this key implicitly implies
    /// [`run_at_load`](Self::run_at_load), causing `launchd` to speculatively
    /// launch the job.
    pub keep_alive: Option<KeepAlive>,

    /// Whether your job is launched once at the time the job is loaded. The
    /// default is `false`.
    ///
    /// <div class="warning">This key should be avoided, as speculative job launches have an adverse effect on system-boot and user-login scenarios.</div>
    pub run_at_load: Option<bool>,

    /// A directory to `chroot(2)` to before running the job.
    ///
    /// IMPORTANT: iOS and OS X both make significant use of IPC to implement
    /// features. The details of the communication between a client and server
    /// are typically implemented in dynamic library code that is abstracted away
    /// from the caller beneath the API boundary so that the client of a daemon
    /// is not aware of any IPC that is happening.
    ///
    /// So unless the library stack which exists in the jail specified by this
    /// key or a call to `chroot(2)` is identical to the one shipping on the
    /// system, there is no guarantee that a process running in that jail will
    /// know how to communicate with the daemons on the system. Mismatches in the
    /// library stack between the jail and the system can manifest as random
    /// failures, hangs and crashes.
    ///
    /// For these reasons, it is highly recommended that you avoid making use
    /// of this key unless you have taken special precautions to ensure that
    /// the job in question never attempts any IPC by setting the
    /// `XPC_NULL_BOOTSTRAP` environment variable to a value of "1". Note that
    /// even if you have done this, you must also take special care to
    /// propagate this environment  variable to any child processes your job
    /// may spawn through `fork(2)` or `posix_spawn(2)`.  And even if you have
    /// done that, there is no guarantee that any subprocesses spawned by your
    /// child processes will take care to do the same thing unless you
    /// completely control all possible chains of execution, which is unlikely.
    pub root_directory: Option<String>,

    /// A directory to `chdir(2)` to before running the job.
    pub working_directory: Option<String>,

    /// Additional environmental variables to be set before running the job.
    ///
    /// Each key in the dictionary is the name of an environment variable, with
    /// the corresponding value being a string representing the desired value.
    ///
    /// NOTE: Values other than strings will be ignored.
    pub environment_variables: Option<HashMap<String, String>>,

    /// What value should be passed to `umask(2)` before running the job.
    ///
    /// If the value specified is an integer, it must be a decimal
    /// representation of the desired umask, as property lists do not support
    /// encoding integers in octal. If a string is given, the string will be
    /// converted into an integer as per the rules described in `strtoul(3)`,
    /// and an octal value may be specified by prefixing the string with a '0'.
    /// If a string that does not cleanly convert to an integer is specified,
    /// the behavior will be to set a `umask(2)` according to the `strtoul(3)`
    /// parsing rules.
    pub umask: Option<StringOrF32>,

    /// The recommended idle time out (in seconds) to pass to the job.
    ///
    /// Jobs seeking to exit when idle should use the
    /// [`enable_pressured_exit`](Self::enable_pressured_exit) key to opt into
    /// the system mechanism for reclaiming killable jobs under memory
    /// pressure.
    #[deprecated(note = "This key never did anything interesting and is no longer implemented.")]
    time_out: Option<u32>,

    /// The amount of time `launchd` waits between sending the `SIGTERM` signal
    /// and before sending a `SIGKILL` signal when the job is to be stopped.
    ///
    /// The default value is system-defined. The value zero is interpreted as
    /// infinity and should not be used, as it can stall system shutdown
    /// forever.
    pub exit_time_out: Option<u32>,

    /// Lets one override the default throttling policy imposed on jobs by
    /// `launchd`.
    ///
    /// The value is in seconds, and by default, jobs will not be
    /// spawned more than once every 10 seconds. The principle behind this is
    /// that jobs should linger around just in case they are needed again in
    /// the near future. This not only reduces the latency of responses, but it
    /// encourages developers to amortize the cost of program invocation.
    pub throttle_interval: Option<u32>,

    /// Whether `initgroups(3)` should initialize the group list for the job.
    ///
    /// The default is `true`. It will be ignored if
    /// [`user_name`](Self::user_name) is not set. Note that for agents,
    /// [`user_name`](Self::user_name) is ignored.
    pub init_groups: Option<bool>,

    /// Causes the job to be started if any one of the listed paths are
    /// modified.
    ///
    /// <div class="warning">
    ///     Use of this key is highly discouraged, as filesystem event
    ///     monitoring is highly race-prone, and it is entirely possible for
    ///     modifications to be missed. When modifications are caught, there is no
    ///     guarantee that the file will be in a consistent state when the job is
    ///     launched.
    /// </div>
    pub watch_paths: Option<Vec<String>>,

    /// Keeps the job alive as long as the directory or directories specified
    /// are not empty.
    pub queue_directories: Option<Vec<String>>,

    /// Causes the job to be started every time a filesystem is mounted.
    pub start_on_mount: Option<bool>,

    /// Causes the job to be started every N seconds.
    ///
    /// If the system is asleep during the time of the next scheduled interval
    /// firing, that interval will be missed due to shortcomings in
    /// `kqueue(3)`. If the job is running during an interval firing, that
    /// interval firing will likewise be missed.
    pub start_interval: Option<u32>,

    /// Causes the job to be started every calendar interval as specified.
    ///
    /// Missing arguments are considered to be wildcard. The semantics are
    /// similar to `crontab(5)` in how firing dates are specified. Multiple
    /// dictionaries may be specified in an array to schedule multiple calendar
    /// intervals.
    ///
    /// Unlike `cron` which skips job invocations when the computer is asleep,
    /// `launchd` will start the job the next time the computer wakes up.  If
    /// multiple intervals transpire before the computer is woken, those events
    /// will be coalesced into one event upon wake from sleep.
    ///
    /// Note that [`start_interval`](Self::start_interval) and
    /// [`start_calendar_interval`](Self::start_calendar_interval) are not
    /// aware of each other. They are evaluated completely independently by the
    /// system.
    pub start_calendar_interval: Option<Vec<CalendarInterval>>,

    /// The given path should be mapped to the job's `stdin(4)`, and the
    /// contents of that file will be readable from the job's `stdin(4)`.
    ///
    /// If the file does not exist, no data will be delivered to the process'
    /// `stdin(4)`.
    pub standard_in_path: Option<String>,

    /// The given path should be mapped to the job's `stdout(4)`, and any
    /// writes to the job's `stdout(4)` will go to the given file.
    ///
    /// If the file does not exist, it will be created with writable
    /// permissions and ownership reflecting the user and/or group specified as
    /// [`user_name`](Self::user_name) and/or [`group_name`](Self::group_name),
    /// respectively (if set) and permissions reflecting the `umask(2)`
    /// specified by [`umask`](Self::umask), if set.
    pub standard_out_path: Option<String>,

    /// The given path should be mapped to the job's `stderr(4)`, and any
    /// writes to the job's `stderr(4)` will go to the given file.
    ///
    /// Note that this file is opened as readable and writable as mandated by
    /// the POSIX specification for unclear reasons. If the file does not
    /// exist, it will be created with ownership reflecting the user and/or
    /// group specified as [`user_name`](Self::user_name) and/or
    /// [`group_name`](Self::group_name), respectively (if set) and permissions
    /// reflecting the `umask(2)` specified by [`umask`](Self::umask), if set.
    pub standard_error_path: Option<String>,

    /// `launchd` should adjust its log mask temporarily to `LOG_DEBUG` while
    /// dealing with this job.
    pub debug: Option<bool>,

    /// `launchd` should launch the job in a suspended state so that a debugger
    /// can be attached to the process as early as possible (at the first
    /// instruction).
    pub wait_for_debugger: Option<bool>,

    /// Resource limits to be imposed on the job. These adjust variables set with
    /// `setrlimit(2)`.
    pub soft_resource_limits: Option<ResourceLimits>,

    /// Resource limits to be imposed on the job. These adjust variables set with
    /// `setrlimit(2)`.
    pub hard_resource_limits: Option<ResourceLimits>,

    /// What `nice(3)` value should be applied to the daemon.
    pub nice: Option<i8>,

    /// Describes, at a high level, the intended purpose of the job.
    ///
    /// The system will apply resource limits based on what kind of job it is.
    /// If left unspecified, the system will apply light resource limits to the
    /// job, throttling its CPU usage and I/O bandwidth. This classification
    /// is preferable to using
    /// [`hard_resource_limits`](Self::hard_resource_limits),
    /// [`soft_resource_limits`](Self::soft_resource_limits) and
    /// [`nice`](Self::nice).
    pub process_type: Option<ProcessType>,

    /// When a job dies, `launchd` kills any remaining processes with the same
    /// process group ID as the job. Setting this to `true` disables that
    /// behavior.
    pub abandon_process_group: Option<bool>,

    /// Whether the kernel should consider this daemon to be low priority when
    /// doing filesystem I/O.
    #[serde(rename = "LowPriorityIO")]
    pub low_priority_io: Option<bool>,

    /// Whether the kernel should consider this daemon to be low priority when
    /// doing filesystem I/O when the process is throttled with the
    /// Darwin-background classification.
    #[serde(rename = "LowPriorityBackgroundIO")]
    pub low_priority_background_io: Option<bool>,

    /// The dataless file materialization policy.
    ///
    /// Setting this to `true` causes dataless files to be materialized.
    /// Setting this to `false` causes dataless files to not be materialized.
    /// If not set, the default system policy for dataless files will be used.
    /// See `setiopolicy_np(3)`.
    pub materialized_dataless_files: Option<bool>,

    /// Specifies whether the job can only be run once and only
    /// once.  In other words, if the job cannot be safely respawned without a
    /// full machine reboot, then set this key to be true.
    launch_only_once: Option<bool>,

    /// This optional key is used to specify Mach services to be registered with
    /// the Mach bootstrap namespace. Each key in this dictionary should be the
    /// name of a service to be advertised. The value of the key must be a
    /// boolean and set to true or a dictionary in order for the service to be
    /// advertised.
    mach_services: Option<HashMap<String, MachServiceConfig>>,

    /// This optional key is used to specify launch on demand sockets that can be
    /// used to let launchd know when to run the job. The job must check-in to
    /// get a copy of the file descriptors using the launch_activate_socket(3)
    /// API.  The keys of the top level Sockets dictionary can be anything. These
    /// keys are meant for the application developer to associate which socket
    /// descriptors correspond to which application level protocols (e.g. http
    /// vs. ftp vs. DNS...).
    ///
    /// The parameters below are used as inputs to call getaddrinfo(3).
    sockets: Option<HashMap<String, SocketValue>>,

    /// Specifies higher-level event types to be used as launch-on-demand event
    /// sources.  Each sub-dictionary defines events for a particular event
    /// subsystem, such as "com.apple.iokit.matching", which can be used to
    /// launch jobs based on the appearance of nodes in the IORegistry. Each
    /// dictionary within the sub-dictionary specifies an event descriptor that
    /// is specified to each event subsystem. With this key, the job promises to
    /// use the xpc_set_event_stream_handler(3) API to consume events. See
    /// xpc_events(3) for more details on event sources.
    launch_events: Option<HashMap<String, HashMap<String, HashMap<String, String>>>>,

    /// This key was a hack for jobs which could not properly keep track of their
    /// clients and is no longer implemented.
    hopefully_exits_last: Option<String>,

    /// This key was a hack for jobs which could not properly keep track of their
    /// clients and is no longer implemented.
    hopefully_exits_first: Option<String>,

    /// This key specifies that the job should be spawned into a new security
    /// audit session rather than the default session for the context is belongs
    /// to. See auditon(2) for details.
    session_create: Option<bool>,

    /// This optional key controls the behavior of timers created by the job. By
    /// default on OS X Mavericks version 10.9 and later, timers created by
    /// launchd jobs are coalesced. Batching the firing of timers with similar
    /// deadlines improves the overall energy efficiency of the system. If this
    /// key is set to true, timers created by the job will opt into less
    /// efficient but more precise behavior and not be coalesced with other
    /// timers. This key may have no effect if the job's ProcessType is not set
    /// to Interactive.
    legacy_timers: Option<bool>,

    /// This optional key indicates which bundles are associated with this job in
    /// the System Settings Login Items UI. If an app installs a legacy plist the
    /// plist should include this key with a value of the app's bundle
    /// identifier.
    associated_bundle_identifiers: Option<StringOrVec>,
}

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct InetdCompatibility {
    ///  This flag corresponds to the "wait" or "nowait" option of inetd. If
    /// true, then the listening socket is passed via the stdio(3) file
    /// descriptors. If false, then accept(2) is called on behalf of the
    /// job, and the result is passed via the stdio(3) descriptors.
    wait: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SessionType {
    Single(String),
    Many(Vec<String>),
}

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct CalendarInterval {
    /// The minute (0-59) on which this job will be run.
    minute: Option<u32>,

    /// The hour (0-23) on which this job will be run.
    hour: Option<u32>,

    /// The day of the month (1-31) on which this job will be run.
    day: Option<u32>,

    /// The weekday on which this job will be run (0 and 7 are Sunday). If
    /// both Day and Weekday are specificed, then the job will be started
    /// if either one matches the current date.
    weekday: Option<u8>,

    /// The month (1-12) on which this job will be run.
    month: Option<u8>,
}

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

#[derive(Clone, Deserialize, Serialize)]
pub enum ProcessType {
    /// Background jobs are generally processes that do work that was not
    /// directly requested by the user. The resource limits applied to
    /// Background jobs are intended to prevent them from disrupting the
    /// user experience.
    Background,

    /// Standard jobs are equivalent to no ProcessType being set.
    Standard,

    /// Adaptive jobs move between the Background and Interactive
    /// classifications based on activity over XPC connections. See
    /// xpc_transaction_begin(3) for details.
    Adaptive,

    /// Interactive jobs run with the same resource limitations as apps,
    /// that is to say, none. Interactive jobs are critical to maintaining
    /// a responsive user experience, and this key should only be used if
    /// an app's ability to be responsive depends on it, and cannot be made
    /// Adaptive.
    Interactive,
}
