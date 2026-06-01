use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        // TODO: construct MyBox.
        let _ = value;
        todo!("construct MyBox")
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO: return a reference to the inner value.
        todo!("return &self.0")
    }
}

pub struct Tracked {
    pub name: String,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        // TODO: print a message that includes self.name.
    }
}

pub fn hello(name: &str) -> String {
    format!("hello, {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derefs_to_inner_value() {
        let value = MyBox::new(5);
        assert_eq!(*value, 5);
    }

    #[test]
    fn deref_coercion_allows_str_argument() {
        let name = MyBox::new(String::from("rust"));
        assert_eq!(hello(&name), "hello, rust");
    }
}
