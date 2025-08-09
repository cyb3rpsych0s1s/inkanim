use std::borrow::Cow;

// use crate::widget::{
//     font::{textHorizontalAlignment, textVerticalAlignment},
//     inkTextWidget,
// };

pub trait Value {
    fn value(&self) -> Cow<'_, str>;
}

// pub trait Instantiate {
//     fn instantiate(&self, name: &str) -> Cow<'_, str>;
// }

// pub trait Setter {
//     fn setter(&self, name: &str, field: &str, value: &str) -> Cow<'_, str>;
// }

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
