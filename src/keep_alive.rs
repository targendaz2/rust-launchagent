use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum KeepAlive {
    Bool(bool),
    Object {
        /// If true, the job will be restarted as long as the program exits and
        /// with an exit status of zero.  If false, the job will be restarted
        /// in the inverse condition.  This key implies that "RunAtLoad" is set
        /// to true, since the job needs to run at least once before an exit
        /// status can be determined.
        successful_exit: Option<bool>,

        /// This key is no longer implemented as it never acted how most users
        /// expected.
        network_state: Option<bool>,

        /// Each key in this dictionary is a file-system path. If the value of
        /// the key is true, then the job will be kept alive as long as the
        /// path exists.  If false, the job will be kept alive in the inverse
        /// condition. The intent of this feature is that two or more jobs may
        /// create semaphores in the file- system namespace. The following
        /// example keeps the job alive as long as the file /path/to/file
        /// exists.
        ///
        /// IMPORTANT: Filesystem monitoring mechanisms are inherently race-
        /// prone and lossy. This option should be avoided in favor of demand-
        /// based alternatives using IPC.
        path_state: Option<HashMap<String, bool>>,

        /// Each key in this dictionary is the name of another job. If the
        /// value is true, then the job will be kept alive as long as one of
        /// the specified other jobs is loaded in launchd(8).
        ///
        /// NOTE: This key only evaluates whether the job is loaded, not
        /// whether it is running. Use of this key is highly discouraged. If
        /// multiple jobs need to coordinate coordinate their lifecycles, they
        /// should establish contracts using IPC.
        other_job_enabled: Option<HashMap<String, bool>>,

        /// If true, the the job will be restarted as long as it exited due to
        /// a signal which is typically associated with a crash (SIGILL,
        /// SIGSEGV, etc.). If false, the job will be restarted in the inverse
        /// condition.
        crashed: Option<bool>,
    },
}
