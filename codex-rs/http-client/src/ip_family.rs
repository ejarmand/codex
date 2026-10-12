//! Process-wide restriction on which IP family outbound connections may use.
//!
//! Set once from configuration (`ip_family` in config.toml) before clients are built. Covers
//! reqwest clients built through [`crate::HttpClientBuilder`] and direct WebSocket dials.

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

/// IP family that outbound connections are restricted to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IpFamily {
    /// Use whatever the resolver returns.
    #[default]
    Any,
    Ipv4,
    Ipv6,
}

static IP_FAMILY: AtomicU8 = AtomicU8::new(0);

/// Restricts outbound connections made after this call to `family`.
pub fn set_ip_family(family: IpFamily) {
    let value = match family {
        IpFamily::Any => 0,
        IpFamily::Ipv4 => 1,
        IpFamily::Ipv6 => 2,
    };
    IP_FAMILY.store(value, Ordering::Relaxed);
}

pub fn ip_family() -> IpFamily {
    match IP_FAMILY.load(Ordering::Relaxed) {
        1 => IpFamily::Ipv4,
        2 => IpFamily::Ipv6,
        _ => IpFamily::Any,
    }
}

/// Unspecified local address that makes hyper connect only to addresses of the configured family.
pub(crate) fn local_address() -> Option<IpAddr> {
    match ip_family() {
        IpFamily::Any => None,
        IpFamily::Ipv4 => Some(Ipv4Addr::UNSPECIFIED.into()),
        IpFamily::Ipv6 => Some(Ipv6Addr::UNSPECIFIED.into()),
    }
}

/// Drops resolved addresses outside the configured family. Returns them unchanged when that
/// would leave none, so a host without the configured family still connects.
pub fn filter_addresses(addresses: Vec<SocketAddr>) -> Vec<SocketAddr> {
    let keep: fn(&SocketAddr) -> bool = match ip_family() {
        IpFamily::Any => return addresses,
        IpFamily::Ipv4 => SocketAddr::is_ipv4,
        IpFamily::Ipv6 => SocketAddr::is_ipv6,
    };
    if addresses.iter().any(keep) {
        addresses.into_iter().filter(keep).collect()
    } else {
        addresses
    }
}
