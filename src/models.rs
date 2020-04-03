use nix::ifaddrs::getifaddrs;
use nix::sys::socket::{InetAddr, SockAddr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Interface {
    name: String,
    addr: String,
    netmask: String,
}

impl Interface {
    pub fn new(name: &str, addr: &str, netmask: &str) -> Interface {
        Interface {
            name: name.to_owned(), addr: addr.to_owned(), netmask: netmask.to_owned()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait Interfaces {
    fn all(&self) -> Result<Vec<Interface>, String>;
    fn create(&self, name: &str) -> Result<Interface, String>;
    fn delete(&self, name: &str) -> Result<bool, String>;
    fn get(&self, name: &str) -> Result<Option<Interface>, String>;
    fn modify(&self, iface: Interface) -> Result<bool, String>;
}

pub struct SystemInterfaces {}

impl SystemInterfaces {
    pub fn new() -> SystemInterfaces {
        SystemInterfaces {}
    }
}

fn is_ipv4_addr(addr: &Option<SockAddr>) -> bool {
    match addr {
        Some(SockAddr::Inet(InetAddr::V4(_))) => true,
        _ => false,
    }
}

impl Interfaces for SystemInterfaces {
    fn all(&self) -> Result<Vec<Interface>, String> {
        match getifaddrs() {
            Ok(addrs) => Ok(addrs
                .filter(|addr| is_ipv4_addr(&addr.address))
                .map(|addr| Interface::new(
                    &addr.interface_name,
                    &addr.address.unwrap().to_str(),
                    &addr.netmask.unwrap().to_str(),
                ))
                .collect()),
            Err(err) => Err(format!("getifaddrs() failed: {}", err)),
        }
    }

    fn create(&self, name: &str) -> Result<Interface, String> {
        Err(format!("TODO: ip link create {}", name))
    }

    fn delete(&self, name: &str) -> Result<bool, String> {
        Err(format!("TODO: ip link delete {}", name))
    }

    fn get(&self, name: &str) -> Result<Option<Interface>, String> {
        match getifaddrs() {
            Ok(addrs) => {
                for ifaddr in addrs.filter(|addr| is_ipv4_addr(&addr.address)) {
                    if ifaddr.interface_name == name {
                        return Ok(Some(Interface::new(
                            &ifaddr.interface_name,
                            &ifaddr.address.unwrap().to_str(),
                            &ifaddr.netmask.unwrap().to_str(),
                        )));
                    }
                }
                Ok(None)
            }
            Err(err) => Err(format!("getifaddrs() failed: {}", err)),
        }
    }

    fn modify(&self, iface: Interface) -> Result<bool, String> {
        Err(format!("TODO: ip link modify {:?}", iface))
    }
}

