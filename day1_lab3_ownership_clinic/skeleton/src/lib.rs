pub fn describe_device(name: &str) -> String {
    // string slice로 바꿔줘서 나중에 다시 저 string을 쓸수있도록 처리
    
    // TODO: this function should not take ownership.
    format!("device={name}")
}

pub fn total(values: &[i32]) -> i32 {
    // total 또한 vector를 저장할 필요가 없다.
    // TODO: this function should not take ownership of the vector.
    values.iter().sum()
}

pub fn append_ready(log: &mut String) {
    // TODO: change this function so it updates the caller's String in place.
    log.push_str(" -> ready");
}

pub fn summarize_log(log: &mut String) -> String {
    // TODO: avoid overlapping mutable and immutable borrows.
    {
        let writer = &mut *log;
        writer.push_str(" -> checked");
    }
    format!("{log} (len={})", log.len())
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
