use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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

    /// The weekday on which this job will be run (0 and 7 are Sunday).
    ///
    /// If both [`day`](Self::day) and [`weekday`](Self::weekday) are
    /// specificed, then the job will be started if either one matches the
    /// current date.
    weekday: Option<u8>,

    /// The month (1-12) on which this job will be run.
    month: Option<u8>,
}
