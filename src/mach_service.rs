use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::defaults::default_false;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum MachServiceConfig {
    Bool(bool),
    Object(MachService),
}

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct MachService {
    /// The default value for this key is false, and so the port is
    /// recycled, thus leaving clients to remain oblivious to the demand
    /// nature of the job. If the value is set to true, clients receive
    /// port death notifications when the job lets go of the receive right.
    /// The port will be recreated atomically with respect to
    /// bootstrap_look_up() calls, so that clients can trust that after
    /// receiving a port-death notification, the new port will have already
    /// been recreated. Setting the value to true should be done with care.
    /// Not all clients may be able to handle this behavior. The default
    /// value is false.
    ///
    /// Note that this option is not compatible with xpc(3), which
    /// automatically handles notifying clients of interrupted connections
    /// and server death.
    #[serde(default = "default_false")]
    reset_at_close: bool,

    /// Reserve the name in the namespace, but cause bootstrap_look_up() to
    /// fail until the job has checked in with launchd.
    ///
    /// This option is incompatible with xpc(3), which relies on the
    /// constant availability of services. This option also encourages
    /// polling for service availability and is therefore generally
    /// discouraged. Future implementations will penalize use of this
    /// option in subtle and creative ways.
    ///
    /// Jobs can dequeue messages from the MachServices they advertised
    /// with xpc_connection_create_mach_service(3) or bootstrap_check_in()
    /// API (to obtain the underlying port's receive right) and the Mach
    /// APIs to dequeue messages from that port.
    #[serde(default = "default_false")]
    hide_until_check_in: bool,
}
