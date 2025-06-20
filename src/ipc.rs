use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{defaults::default_false, unions::StringOrU32};

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct InetdCompatibility {
    /// Corresponds to the "wait" or "nowait" option of `inetd`.
    ///
    /// If `true`, then the listening socket is passed via the `stdio(3)` file
    /// descriptors. If `false`, then `accept(2)` is called on behalf of the
    /// job, and the result is passed via the `stdio(3)` descriptors.
    pub wait: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MachService {
    Bool(bool),

    #[serde(rename_all = "PascalCase")]
    Object {
        /// If `false`, the port is recycled, thus leaving clients to remain
        /// oblivious to the demand nature of the job. If `true`, clients
        /// receive port death notifications when the job lets go of the
        /// receive right. The port will be recreated atomically with respect
        /// to `bootstrap_look_up()` calls, so that clients can trust that
        /// after receiving a port-death notification, the new port will have
        /// already been recreated. Setting the value to `true` should be done
        /// with care. Not all clients may be able to handle this behavior.
        ///
        /// Note that this option is not compatible with `xpc(3)`, which
        /// automatically handles notifying clients of interrupted connections
        /// and server death.
        #[serde(default = "default_false")]
        reset_at_close: bool,

        /// Reserve the name in the namespace, but cause `bootstrap_look_up()`
        /// to fail until the job has checked in with `launchd`.
        ///
        /// This option is incompatible with `xpc(3)`, which relies on the
        /// constant availability of services. This option also encourages
        /// polling for service availability and is therefore generally
        /// discouraged. Future implementations will penalize use of this
        /// option in subtle and creative ways.
        ///
        /// Jobs can dequeue messages from the MachServices they advertised
        /// with `xpc_connection_create_mach_service(3)` or
        /// `bootstrap_check_in()` API (to obtain the underlying port's receive
        /// right) and the Mach APIs to dequeue messages from that port.
        #[serde(default = "default_false")]
        hide_until_check_in: bool,
    },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SocketValue {
    Single(Socket),
    Many(Vec<Socket>),
}

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct Socket {
    /// What type of socket to create.
    #[serde(rename = "SockType")]
    pub socket_type: Option<SocketType>,

    /// Whether `listen(2)` or `connect(2)` should be called on the created
    /// file descriptor.
    ///
    /// The default is `true`, to listen for new connections.
    #[serde(rename = "SockPassive")]
    pub passive: Option<bool>,

    /// The node to `connect(2)` or `bind(2)` to.
    #[serde(rename = "SockNodeName")]
    pub node_name: Option<String>,

    /// The service on the node to `connect(2)` or `bind(2)` to.
    ///
    /// It may be a port number represented as an integer or a service name
    /// represented as a string ("ssh", "telnet", etc.)
    #[serde(rename = "SockServiceName")]
    pub service_name: Option<StringOrU32>,

    /// Specifically request that "IPv4" or "IPv6" socket(s) be created.
    ///
    /// An additional option, "IPv4v6" indicates that a single socket that
    /// listens for both IPv4 and IPv6 connections should be created.
    #[serde(rename = "SockFamily")]
    pub family: Option<SocketFamily>,

    /// The protocol to be passed to `socket(2)`.
    #[serde(rename = "SockProtocol")]
    pub protocol: Option<SocketProtocol>,

    /// Specifies the path to `connect(2)` or `bind(2)` to.
    ///
    /// Implies [`family`](Self::family) is set to
    /// [`Unix`](SocketFamily::Unix).
    #[serde(rename = "SockPathName")]
    pub path_name: Option<String>,

    /// A variant of [`path_name`](Self::path_name). Instead of binding to a
    /// known path, a securely generated socket is created and the path is
    /// assigned to the environment variable that is inherited by all jobs
    /// spawned in the job's context.
    pub secure_socket_with_key: Option<String>,

    /// The user ID that should be the domain socket's owner.
    #[serde(rename = "SockPathOwner")]
    pub path_owner: Option<u32>,

    /// The group ID that should be set as the domain socket's group.
    #[serde(rename = "SockPathGroup")]
    pub path_group: Option<u32>,

    /// The mode of the socket.
    ///
    /// <div class="warning">
    ///     Known bug: Property lists don't support octal, so please convert
    ///     the value to decimal.
    /// </div>
    #[serde(rename = "SockPathMode")]
    pub path_mode: Option<f32>,

    /// Request that the service be registered with the the Bonjour subsystem.
    ///
    /// If the value is boolean, the service name is inferred from
    /// [`service_name`](Self::service_name).
    pub bonjour: Option<Bonjour>,

    /// Request that the datagram socket join a multicast group.
    ///
    /// If the value is a hostname, then `getaddrinfo(3)` will be used to join
    /// the correct multicast address for a given socket family.  If an
    /// explicit IPv4 or IPv6 address is given, it is required that
    /// [`family`](Self::family) also be set, otherwise the results are
    /// undefined.
    pub multicast_group: Option<String>,
}

/// The type of socket to create.
#[derive(Clone, Deserialize, Serialize)]
pub enum SocketType {
    Stream,
    Dgram,
    Seqpacket,
}

/// The family of socket to create.
#[derive(Clone, Deserialize, Serialize)]
pub enum SocketFamily {
    IPv4,
    IPv6,
    IPv4v6,
    Unix,
}

/// The protocol to use for the socket.
#[derive(Clone, Deserialize, Serialize)]
pub enum SocketProtocol {
    TCP,
    UDP,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Bonjour {
    Bool(bool),
    String(String),
    Array(Vec<String>),
}
