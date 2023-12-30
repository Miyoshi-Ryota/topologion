use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub nics: Vec<Rc<Nic>>,
}

#[derive(Debug)]
pub struct Nic {
    pub name: String,
    pub device: Weak<RefCell<Device>>,
}

#[derive(Debug)]
pub struct Link {
    pub nic1: Rc<Nic>,
    pub nic2: Rc<Nic>,
}

#[derive(Debug)]
pub struct Topology {
    pub devices: Vec<Rc<RefCell<Device>>>,
    pub links: Vec<Link>,
}

impl Topology {
    pub fn new() -> Self {
        Topology {
            devices: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn add_device(&mut self, name: String, nic_names: Vec<String>) {
        let device = Rc::new(RefCell::new(Device { name, nics: vec![] }));

        for nic_name in nic_names {
            let nic = Rc::new(Nic {
                name: nic_name,
                device: Rc::downgrade(&device),
            });
            device.borrow_mut().nics.push(nic);
        }

        self.devices.push(device);
    }

    pub fn connect(&mut self, nic1: Rc<Nic>, nic2: Rc<Nic>) {
        let link = Link { nic1, nic2 };
        self.links.push(link);
    }
}
