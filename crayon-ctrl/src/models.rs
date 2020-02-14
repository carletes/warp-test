use nix::ifaddrs::{getifaddrs, InterfaceAddress};

#[derive(Debug)]
pub struct Interface {
    pub dev: String,
    addr: String,
    prefix: u8,
}

impl Interface {
    fn from_ifaddr(addr: InterfaceAddress) -> Interface {
        Interface {
            dev: addr.interface_name,
            addr: "pepe".to_string(),
            prefix: 42,
        }
    }
}

pub trait Interfaces {
    fn add(&mut self, iface: Interface) -> Result<(), String>;
    fn get(&self, dev: &str) -> Option<Interface>;
    fn list(&self) -> Result<Vec<Interface>, String>;
}

pub struct SysInterfaces {}

impl Interfaces for SysInterfaces {
    fn add(&mut self, iface: Interface) -> Result<(), String> {
        Err(format!("Not implemented: add({:?})", iface))
    }

    fn get(&self, dev: &str) -> Option<Interface> {
        match getifaddrs() {
            Ok(addrs) => {
                for addr in addrs {
                    if addr.interface_name == dev {
                        return Some(Interface::from_ifaddr(addr));
                    }
                }

                None
            }
            Err(_) => None,
        }
    }

    fn list(&self) -> Result<Vec<Interface>, String> {
        match getifaddrs() {
            Ok(addrs) => {
                let mut ret = Vec::new();
                for addr in addrs {
                    ret.push(Interface::from_ifaddr(addr));
                }
                Ok(ret)
            }
            Err(err) => Err(format!("getifaddrs() failed: {}", err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Interfaces, SysInterfaces};

    #[test]
    fn all_interfaces() {
        let system = SysInterfaces {};
        let ifaces = system.list().unwrap();
        assert!(!ifaces.is_empty());
    }
}
