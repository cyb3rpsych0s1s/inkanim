mod ink;
use ink::widget::Widget;
pub use ink::*;

pub trait RedsValue {
    fn reds_value(&self) -> Option<String>;
}

pub trait RedsWidgetLeaf {
    fn reds_widget_leaf(&self, name: &str, parent: Option<&str>) -> String;
}

pub trait RedsWidgetCompound {
    fn reds_widget_compound(&self, name: &str, parent: Option<&str>) -> String;
}

pub trait RedsWidget {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String;
}

impl RedsWidget for &dyn RedsWidgetLeaf {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        self.reds_widget_leaf(name, parent)
    }
}

impl RedsWidget for &dyn RedsWidgetCompound {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        self.reds_widget_compound(name, parent)
    }
}

impl InkWrapper<Widget> {
    pub fn name(&self) -> Option<&str> {
        self.data.name()
    }
}

impl RedsWidget for &InkWrapper<Widget> {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        self.data.reds_widget(name, parent)
    }
}

impl RedsWidget for Widget {
    fn reds_widget(&self, name: &str, parent: Option<&str>) -> String {
        match self {
            Widget::inkMultiChildren(x) => x.reds_widget(name, parent),
            Widget::inkCanvasWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkHorizontalPanelWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkVerticalPanelWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkScrollAreaWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkUniformGridWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkVirtualCompoundWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkFlexWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkCacheWidget(x) => x.reds_widget_compound(name, parent),
            Widget::inkTextWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkImageWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkVideoWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkMaskWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkBorderWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkShapeWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkCircleWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkRectangleWidget(x) => x.reds_widget_leaf(name, parent),
            Widget::inkVectorGraphicWidget(x) => x.reds_widget_leaf(name, parent),
        }
    }
}

impl<T> RedsValue for Vec<T>
where
    T: RedsValue,
{
    fn reds_value(&self) -> Option<String> {
        Some(
            self.iter()
                .map(|x| x.reds_value().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(""),
        )
    }
}

impl<T> RedsValue for Option<T>
where
    T: RedsValue,
{
    fn reds_value(&self) -> Option<String> {
        self.as_ref().and_then(|x| x.reds_value())
    }
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

impl RedsValue for u32 {
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
