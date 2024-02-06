mod ink;
pub use ink::*;

pub enum Reds {
    Default,
    OneLiner(String),
    MultiStep { name: String, instantiation: String },
}

pub trait RedsValue {
    fn reds_value(&self) -> Option<String>;
}

pub trait RedsWidget {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String;
}

impl RedsValue for f32 {
    fn reds_value(&self) -> Option<String> {
        if self == &self.trunc() {
            Some(format!("{}.", self))
        } else {
            Some(format!("{}", self))
        }
    }
}
