mod ink;
pub use ink::*;

pub trait RedsValue {
    fn reds_value(&self) -> Option<String>;
}

pub trait RedsWidget {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String;
}
