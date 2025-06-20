use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
