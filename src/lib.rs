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

impl RedsValue for i32 {
    fn reds_value(&self) -> Option<String> {
        Some(format!("{}", self.clone()))
    }
}

impl RedsValue for u16 {
    fn reds_value(&self) -> Option<String> {
        Some(format!("{}", self.clone()))
    }
}

impl RedsValue for bool {
    fn reds_value(&self) -> Option<String> {
        if !self {
            None
        } else {
            Some("true".to_string())
        }
    }
}

impl RedsValue for String {
    fn reds_value(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl RedsValue for Name {
    fn reds_value(&self) -> Option<String> {
        match (
            self.r#type.as_str(),
            self.storage.as_str(),
            self.value.as_str(),
        ) {
            ("ResourcePath", "string", "") => None,
            ("ResourcePath", "string", v) => Some(format!("r\"{v}\"")),
            ("CName", "string", "None") => None,
            ("CName", "string", v) => Some(format!("n\"{v}\"")),
            _ => unreachable!(),
        }
    }
}

impl RedsValue for LocalizationString {
    fn reds_value(&self) -> Option<String> {
        if let Some(ref v) = self.value {
            return match v {
                LocKey::ID(v) if v == &0 => None,
                LocKey::ID(v) => Some(format!("LocKey#{}", v)),
                LocKey::Value(v) if v.as_str() == "null" => None,
                LocKey::Value(v) if v.as_str() == "None" => None,
                LocKey::Value(v) => Some(format!("l\"{}\"", v)),
            };
        }
        None
    }
}
