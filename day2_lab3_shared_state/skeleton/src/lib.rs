use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Device {
    pub name: String,
    pub readings: Vec<i32>,
}

impl Device {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), readings: Vec::new() }
    }
}

pub type SharedDevice = Rc<RefCell<Device>>;

pub fn make_shared_device(name: &str) -> SharedDevice {
    let _ = name;
    todo!("create Rc<RefCell<Device>>")
}

pub fn attach_reading(device: &SharedDevice, value: i32) {
    let _ = (device, value);
    todo!("borrow_mut and push value")
}

pub fn snapshot(device: &SharedDevice) -> String {
    let _ = device;
    todo!("borrow immutably and format name/readings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_shared_device() {
        let device = make_shared_device("ecu");
        let handler_a = Rc::clone(&device);
        let handler_b = Rc::clone(&device);

        attach_reading(&handler_a, 10);
        attach_reading(&handler_b, 20);

        assert_eq!(snapshot(&device), "ecu: [10, 20]");
        assert_eq!(Rc::strong_count(&device), 3);
    }
}
