#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    pub addr: String,
    pub prefix: u8,
}

pub trait Interfaces {
    fn all(&self) -> Vec<Interface>;
    fn create(&mut self, name: &str) -> Result<Interface, String>;
    fn delete(&mut self, name: &str) -> Result<bool, String>;
    fn get(&self, name: &str) -> Result<Option<Interface>, String>;
    fn modify(&self, iface: Interface) -> Result<bool, String>;
}

pub struct SystemInterfaces {}

impl Interfaces for SystemInterfaces {
    fn all(&self) -> Vec<Interface> {
        Vec::new()
    }

    fn create(&mut self, name: &str) -> Result<Interface, String> {
        Err(format!("TODO: ip link create {}", name))
    }

    fn delete(&mut self, name: &str) -> Result<bool, String> {
        Err(format!("TODO: ip link delete {}", name))
    }

    fn get(&self, name: &str) -> Result<Option<Interface>, String> {
        Err(format!("TODO: ip show {:?}", name))
    }

    fn modify(&self, iface: Interface) -> Result<bool, String> {
        Err(format!("TODO: ip link modify {:?}", iface))
    }
}
