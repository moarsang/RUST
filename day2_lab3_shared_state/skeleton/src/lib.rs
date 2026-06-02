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
    Rc::new(RefCell::new(Device::new(name)))
}

pub fn attach_reading(device: &SharedDevice, value: i32) {
    device.borrow_mut().readings.push(value);
}

pub fn snapshot(device: &SharedDevice) -> String {
    let borrowed = device.borrow();
    format!("{}: {:?}", borrowed.name, borrowed.readings)
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
