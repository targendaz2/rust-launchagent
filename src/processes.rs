use serde::{Deserialize, Serialize};

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
