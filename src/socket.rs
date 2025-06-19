use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::common::StringOrInt;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub enum SocketValue {
    Single(Socket),
    Many(Vec<Socket>),
}

#[derive(Builder, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option))]
pub struct Socket {
    /// This optional key tells launchd what type of socket to create. The
    /// default is "stream" and other valid values for this key are "dgram"
    /// and "seqpacket" respectively.
    sock_type: Option<SockType>,

    /// This optional key specifies whether listen(2) or connect(2) should
    /// be called on the created file descriptor. The default is true, to
    /// listen for new connections.
    sock_passive: Option<bool>,

    /// This optional key specifies the node to connect(2) or bind(2) to.
    sock_node_name: Option<String>,

    /// This optional key specifies the service on the node to connect(2)
    /// or bind(2) to. It may be a port number represented as an integer or
    /// a service name represented as a string ("ssh", "telnet", etc.)
    sock_service_name: Option<StringOrInt>,

    /// This optional key can be used to specifically request that "IPv4"
    /// or "IPv6" socket(s) be created. An additional option, "IPv4v6"
    /// indicates that a single socket that listens for both IPv4 and IPv6
    /// connections should be created.
    sock_family: Option<String>,

    /// This optional key specifies the protocol to be passed to socket(2).
    /// The only values understood by this key at the moment are "TCP" and
    /// "UDP".
    sock_protocol: Option<SockProtocol>,

    /// This optional key implies SockFamily is set to "Unix". It specifies
    /// the path to connect(2) or bind(2) to.
    sock_path_name: Option<String>,

    /// This optional key is a variant of SockPathName. Instead of binding
    /// to a known path, a securely generated socket is created and the
    /// path is assigned to the environment variable that is inherited by
    /// all jobs spawned in the job's context.
    secure_socket_with_key: Option<String>,

    /// This optional key specifies the user ID that should be the domain
    /// socket's owner.
    sock_path_owner: Option<u32>,

    /// This optional key specifies the group ID that should be set as the
    /// domain socket's group.
    sock_path_group: Option<u32>,

    /// This optional key specifies the mode of the socket. Known bug:
    /// Property lists don't support octal, so please convert the value to
    /// decimal
    sock_path_mode: Option<u32>,

    /// This optional key can be used to request that the service be
    /// registered with the the Bonjour subsystem. If the value is boolean,
    /// the service name is inferred from the SockServiceName.
    bonjour: Option<BonjourValue>,

    /// This optional key can be used to request that the datagram socket
    /// join a multicast group. If the value is a hostname, then
    /// getaddrinfo(3) will be used to join the correct multicast address
    /// for a given socket family.  If an explicit IPv4 or IPv6 address is
    /// given, it is required that the SockFamily family also be set,
    /// otherwise the results are undefined.
    multicast_group: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
enum SockType {
    Stream,
    Dgram,
    Seqpacket,
}

#[derive(Clone, Deserialize, Serialize)]
enum SockProtocol {
    TCP,
    UDP,
}

#[derive(Clone, Deserialize, Serialize)]
enum BonjourValue {
    Bool(bool),
    String(String),
    Array(Vec<String>),
}
