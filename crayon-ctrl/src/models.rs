use nix::ifaddrs::getifaddrs;

#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    pub addr: String,
    pub netmask: String,
}

pub trait Interfaces {
    fn all(&self) -> Result<Vec<Interface>, String>;
    fn create(&mut self, name: &str) -> Result<Interface, String>;
    fn delete(&mut self, name: &str) -> Result<bool, String>;
    fn get(&self, name: &str) -> Result<Option<Interface>, String>;
    fn modify(&self, iface: Interface) -> Result<bool, String>;
}

pub struct SystemInterfaces {}

impl Interfaces for SystemInterfaces {
    fn all(&self) -> Result<Vec<Interface>, String> {
        match getifaddrs() {
            Ok(addrs) => Ok(addrs
                .map(|addr| Interface {
                    name: addr.interface_name,
                    addr: "127.0.0.1".to_string(),
                    netmask: "255.0.0.0".to_string(),
                })
                .collect()),
            Err(err) => Err(format!("getifaddrs() failed: {}", err)),
        }
    }

    fn create(&mut self, name: &str) -> Result<Interface, String> {
        Err(format!("TODO: ip link create {}", name))
    }

    fn delete(&mut self, name: &str) -> Result<bool, String> {
        Err(format!("TODO: ip link delete {}", name))
    }

    fn get(&self, name: &str) -> Result<Option<Interface>, String> {
        match getifaddrs() {
            Ok(addrs) => {
                for ifaddr in addrs {
                    if ifaddr.interface_name == name {
                        match (ifaddr.address, ifaddr.netmask) {
                            (Some(address), Some(netmask)) => {
                                return Ok(Some(Interface {
                                    name: ifaddr.interface_name,
                                    addr: address.to_string(),
                                    netmask: netmask.to_str(),
                                }))
                            }
                            (_, _) => {
                                return Err(format!(
                                    "Unknown type of address/netmask for interface {}",
                                    name
                                ))
                            }
                        }
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
