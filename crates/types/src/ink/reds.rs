use std::borrow::Cow;

pub trait Type {
    const NAME: &str;
}

pub trait Value {
    fn value(&self) -> Cow<'_, str>;
}

pub trait Instantiate {
    fn instantiate(&self, instance: &str) -> Cow<'_, str>;
}

pub trait Setter {
    const FIELDS: &[&'static str];
    fn setter(&self, instance: &str, field: &str, value: &impl self::Value) -> Cow<'_, str> {
        if !Self::FIELDS.contains(&field) {
            panic!("unknown field {field} for {instance}")
        }
        std::borrow::Cow::Owned(format!("{instance}.{field} = {};", value.value()))
    }
}

impl Type for u32 {
    const NAME: &str = "Uint32";
}

impl Value for u32 {
    fn value(&self) -> Cow<'_, str> {
        std::borrow::Cow::Owned(format!("{self}u"))
    }
}

impl Type for f32 {
    const NAME: &str = "Float";
}

impl Value for f32 {
    fn value(&self) -> Cow<'_, str> {
        if self.fract() == 0.0 {
            return Cow::Owned(format!("{}.0", self));
        }
        self.to_string().into()
    }
}

impl Type for String {
    const NAME: &str = "String";
}

impl Value for String {
    fn value(&self) -> Cow<'_, str> {
        self.into()
    }
}

// pub trait ScriptName {
//     const NAME: &str;
// }

// impl ScriptName for inkTextWidget {
//     const NAME: &str = "inkText";
// }

// macro_rules! scriptname {
//     ($it:ty) => {
//         impl self::ScriptName for $it {
//             const NAME: &str = stringify!($ty);
//         }
//     };
// }

// macro_rules! instantiate_class {
//     ($it:ty) => {
//         impl Instantiate for $it {
//             fn instantiate(&self, name: &str) -> Cow<'_, str> {
//                 use self::ScriptName;
//                 Cow::Owned(format!("let {name}: {} = new {0}();", Self::NAME))
//             }
//         }
//     };
// }

// macro_rules! instantiate_struct {
//     ($it:ty) => {
//         impl Instantiate for $it {
//             fn instantiate(&self, name: &str) -> Cow<'_, str> {
//                 use self::ScriptName;
//                 Cow::Owned{format!("let {name}: {};", Self::NAME)}
//             }
//         }
//     };
// }

// macro_rules! setter {
//     ($it:ty [prop]) => {
//         impl Setter for $it {
//             fn setter(&self, name: &str, field: &str, value: &str) -> Cow<'_, str> {
//                 use self::ScriptName;
//                 Cow::Owned(format!("{name}.{field} = {value};"))
//             }
//         }
//     };
//     ($it:ty [func]) => {
//         impl Setter for $it {
//             fn setter(&self, name: &str, method: &str, value: &str) -> Cow<'_, str> {
//                 use self::ScriptName;
//                 Cow::Owned(format!("{name}.{field} = {value};"))
//             }
//         }
//     };
// }

// scriptname!(textHorizontalAlignment);
// scriptname!(textVerticalAlignment);

// instantiate_class!(inkTextWidget);

// impl Value for textHorizontalAlignment {
//     fn value(&self) -> Cow<'_, str> {
//         use self::ScriptName;
//         Cow::Owned(match self {
//             Self::Left => format!("{}.Left", Self::NAME),
//             Self::Center => format!("{}.Center", Self::NAME),
//             Self::Right => format!("{}.Right", Self::NAME),
//         })
//     }
// }

// impl Value for textVerticalAlignment {
//     fn value(&self) -> Cow<'_, str> {
//         use self::ScriptName;
//         Cow::Owned(match self {
//             Self::Top => format!("{}.Top", Self::NAME),
//             Self::Center => format!("{}.Center", Self::NAME),
//             Self::Bottom => format!("{}.Bottom", Self::NAME),
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{reds::Instantiate, widget::layout::inkMargin};

    #[test]
    fn test_struct() {
        let margin = inkMargin {
            bottom: 0.0,
            left: 2.0,
            right: 10.0,
            top: 200.0,
        };
        assert_eq!(
            margin.instantiate("margin"),
            r#"let margin: inkMargin;
margin.left = 2.0;
margin.top = 200.0;
margin.right = 10.0;
margin.bottom = 0.0;"#
        );
    }
}
