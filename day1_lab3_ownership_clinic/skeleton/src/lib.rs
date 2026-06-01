pub fn describe_device(name: String) -> String {
    // TODO: this function should not take ownership.
    format!("device={name}")
}

pub fn total(values: Vec<i32>) -> i32 {
    // TODO: this function should not take ownership of the vector.
    values.iter().sum()
}

pub fn append_ready(log: String) -> String {
    // TODO: change this function so it updates the caller's String in place.
    let mut log = log;
    log.push_str(" -> ready");
    log
}

pub fn summarize_log(log: &mut String) -> String {
    // TODO: avoid overlapping mutable and immutable borrows.
    let writer = log;
    writer.push_str(" -> checked");
    format!("{writer} (len={})", writer.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_device_available_after_description() {
        let device = String::from("ecu");
        let description = describe_device(&device);
        assert_eq!(description, "device=ecu");
        assert_eq!(device, "ecu");
    }

    #[test]
    fn totals_without_consuming_vector() {
        let values = vec![10, 20, 30];
        assert_eq!(total(&values), 60);
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn appends_in_place() {
        let mut log = String::from("boot");
        append_ready(&mut log);
        assert_eq!(log, "boot -> ready");
    }

    #[test]
    fn summarizes_after_mutation() {
        let mut log = String::from("boot");
        let summary = summarize_log(&mut log);
        assert_eq!(summary, "boot -> checked (len=15)");
    }
}
