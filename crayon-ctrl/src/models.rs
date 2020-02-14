#[derive(Debug)]
pub struct Interface {
    pub dev: String,
    addr: String,
    prefix: u8,
}

pub trait Interfaces {
    fn add(&mut self, iface: Interface) -> Result<(), String>;
    fn get(&self, dev: &str) -> Option<&Interface>;
    fn list(&self) -> Vec<Interface>;
}

struct SysInterfaces {}

impl Interfaces for SysInterfaces {
    fn add(&mut self, iface: Interface) -> Result<(), String> {
        Err(format!("Not implemented: add({:?})", iface))
    }

    fn get(&self, dev: &str) -> Option<&Interface> {
        None
    }

    fn list(&self) -> Vec<Interface> {
        vec![]
    }
}
