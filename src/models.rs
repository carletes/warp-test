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
                .map(|addr| Interface {
                    name: addr.interface_name,
                    addr: addr.address.unwrap().to_str(),
                    netmask: addr.netmask.unwrap().to_str(),
                })
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
                        return Ok(Some(Interface {
                            name: ifaddr.interface_name,
                            addr: ifaddr.address.unwrap().to_str(),
                            netmask: ifaddr.netmask.unwrap().to_str(),
                        }));
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

pub mod test {
    use super::{Interface, Interfaces};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockInterfaces {
        ifaces: RefCell<HashMap<String, Interface>>,
    }

    impl Interfaces for MockInterfaces {
        fn all(&self) -> Result<Vec<Interface>, String> {
            Ok(self.ifaces.borrow().values().cloned().collect())
        }

        fn create(&self, name: &str) -> Result<Interface, String> {
            if self.ifaces.borrow().contains_key(name) {
                Err(format!("{}: Already exists", name))
            } else {
                let k = String::from(name);
                let iface = Interface {
                    name: k.clone(),
                    addr: "127.0.0.1".to_string(),
                    netmask: "255.0.0.0".to_string(),
                };
                let ret = iface.clone();
                self.ifaces.borrow_mut().insert(k, iface);
                Ok(ret)
            }
        }

        fn delete(&self, name: &str) -> Result<bool, String> {
            match self.ifaces.borrow_mut().remove(name) {
                Some(_) => Ok(true),
                None => Ok(false),
            }
        }

        fn get(&self, name: &str) -> Result<Option<Interface>, String> {
            match self.ifaces.borrow().get(name) {
                Some(iface) => Ok(Some(iface.clone())),
                None => Ok(None),
            }
        }

        fn modify(&self, iface: Interface) -> Result<bool, String> {
            Err(format!("TODO: ip link modify {:?}", iface))
        }
    }

    pub fn interfaces() -> Arc<Mutex<impl Interfaces>> {
        Arc::new(Mutex::new(MockInterfaces {
            ifaces: RefCell::new(HashMap::with_capacity(10)),
        }))
    }
}
