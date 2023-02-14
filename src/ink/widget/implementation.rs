use crate::ink::InkWrapper;

use super::{
    inkBorderWidget, inkCacheWidget, inkCanvasWidget, inkCircleWidget, inkFlexWidget,
    inkHorizontalPanelWidget, inkImageWidget, inkMaskWidget, inkMultiChildren, inkRectangleWidget,
    inkScrollAreaWidget, inkShapeWidget, inkTextWidget, inkUniformGridWidget,
    inkVectorGraphicWidget, inkVerticalPanelWidget, inkVideoWidget, inkVirtualCompoundWidget,
    inkWidgetLibraryItem, inkWidgetLibraryItemInstance, inkWidgetLibraryResource, Widget,
    WidgetSummary,
};

macro_rules! impl_ink_children {
    ($ty:ident) => {
        impl InkChildren for $ty {
            fn orphans(&self) -> Vec<Widget> {
                self.children.data.orphans()
            }

            fn children(&self) -> Vec<InkWrapper<Widget>> {
                self.children.data.children()
            }
        }
    };
}

macro_rules! impl_ink_widget {
    ($ty:ident) => {
        impl InkWidget for $ty {
            fn name(&self) -> &str {
                self.name.as_str()
            }
        }
    };
}

macro_rules! impl_leaf_widget {
    ($ty:ident) => {
        impl InkLeafWidget for $ty {}
    };
}

macro_rules! impl_classname {
    ($ty:ident) => {
        impl Classname for $ty {
            fn classname(&self) -> String {
                stringify!($ty).to_string()
            }
        }
    };
}

pub trait Classname {
    fn classname(&self) -> String;
}

pub trait InkWidget {
    fn name(&self) -> &str;
}

pub trait InkChildren {
    fn orphans(&self) -> Vec<Widget>;
    fn children(&self) -> Vec<InkWrapper<Widget>>;
}

pub trait InkLeafWidget: InkWidget {}

pub trait InkCompoundWidget: InkWidget + InkChildren {}

impl<T> InkCompoundWidget for T where T: InkWidget + InkChildren {}

impl InkChildren for inkMultiChildren {
    fn orphans(&self) -> Vec<Widget> {
        self.children.iter().map(|x| x.data.clone()).collect()
    }

    fn children(&self) -> Vec<InkWrapper<Widget>> {
        self.children.to_vec()
    }
}

impl_ink_children!(inkCanvasWidget);
impl_ink_children!(inkHorizontalPanelWidget);
impl_ink_children!(inkVerticalPanelWidget);
impl_ink_children!(inkScrollAreaWidget);
impl_ink_children!(inkUniformGridWidget);
impl_ink_children!(inkVirtualCompoundWidget);
impl_ink_children!(inkFlexWidget);
impl_ink_children!(inkCacheWidget);

impl_ink_widget!(inkCanvasWidget);
impl_ink_widget!(inkHorizontalPanelWidget);
impl_ink_widget!(inkVerticalPanelWidget);
impl_ink_widget!(inkScrollAreaWidget);
impl_ink_widget!(inkUniformGridWidget);
impl_ink_widget!(inkVirtualCompoundWidget);
impl_ink_widget!(inkFlexWidget);
impl_ink_widget!(inkCacheWidget);

impl_ink_widget!(inkTextWidget);
impl_ink_widget!(inkImageWidget);
impl_ink_widget!(inkVideoWidget);
impl_ink_widget!(inkMaskWidget);
impl_ink_widget!(inkBorderWidget);
impl_ink_widget!(inkShapeWidget);
impl_ink_widget!(inkCircleWidget);
impl_ink_widget!(inkRectangleWidget);
impl_ink_widget!(inkVectorGraphicWidget);

impl_leaf_widget!(inkTextWidget);
impl_leaf_widget!(inkImageWidget);
impl_leaf_widget!(inkVideoWidget);
impl_leaf_widget!(inkMaskWidget);
impl_leaf_widget!(inkBorderWidget);
impl_leaf_widget!(inkShapeWidget);
impl_leaf_widget!(inkCircleWidget);
impl_leaf_widget!(inkRectangleWidget);
impl_leaf_widget!(inkVectorGraphicWidget);

impl_classname!(inkMultiChildren);

impl_classname!(inkCanvasWidget);
impl_classname!(inkHorizontalPanelWidget);
impl_classname!(inkVerticalPanelWidget);
impl_classname!(inkScrollAreaWidget);
impl_classname!(inkUniformGridWidget);
impl_classname!(inkVirtualCompoundWidget);
impl_classname!(inkFlexWidget);
impl_classname!(inkCacheWidget);

impl_classname!(inkTextWidget);
impl_classname!(inkImageWidget);
impl_classname!(inkVideoWidget);
impl_classname!(inkMaskWidget);
impl_classname!(inkBorderWidget);
impl_classname!(inkShapeWidget);
impl_classname!(inkCircleWidget);
impl_classname!(inkRectangleWidget);
impl_classname!(inkVectorGraphicWidget);

impl Classname for Widget {
    fn classname(&self) -> String {
        match self {
            Self::inkMultiChildren(widget) => widget.classname(),

            Self::inkCanvasWidget(widget) => widget.classname(),
            Self::inkHorizontalPanelWidget(widget) => widget.classname(),
            Self::inkVerticalPanelWidget(widget) => widget.classname(),
            Self::inkScrollAreaWidget(widget) => widget.classname(),
            Self::inkUniformGridWidget(widget) => widget.classname(),
            Self::inkVirtualCompoundWidget(widget) => widget.classname(),
            Self::inkFlexWidget(widget) => widget.classname(),
            Self::inkCacheWidget(widget) => widget.classname(),

            Self::inkTextWidget(widget) => widget.classname(),
            Self::inkImageWidget(widget) => widget.classname(),
            Self::inkVideoWidget(widget) => widget.classname(),
            Self::inkMaskWidget(widget) => widget.classname(),
            Self::inkBorderWidget(widget) => widget.classname(),
            Self::inkShapeWidget(widget) => widget.classname(),
            Self::inkCircleWidget(widget) => widget.classname(),
            Self::inkRectangleWidget(widget) => widget.classname(),
            Self::inkVectorGraphicWidget(widget) => widget.classname(),
        }
    }
}

impl Widget {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::inkMultiChildren(_) => None,
            Self::inkCanvasWidget(node) => Some(node.name()),
            Self::inkHorizontalPanelWidget(node) => Some(node.name()),
            Self::inkVerticalPanelWidget(node) => Some(node.name()),
            Self::inkScrollAreaWidget(node) => Some(node.name()),
            Self::inkUniformGridWidget(node) => Some(node.name()),
            Self::inkVirtualCompoundWidget(node) => Some(node.name()),
            Self::inkFlexWidget(node) => Some(node.name()),
            Self::inkCacheWidget(node) => Some(node.name()),
            Self::inkTextWidget(node) => Some(node.name()),
            Self::inkImageWidget(node) => Some(node.name()),
            Self::inkVideoWidget(node) => Some(node.name()),
            Self::inkMaskWidget(node) => Some(node.name()),
            Self::inkBorderWidget(node) => Some(node.name()),
            Self::inkShapeWidget(node) => Some(node.name()),
            Self::inkCircleWidget(node) => Some(node.name()),
            Self::inkRectangleWidget(node) => Some(node.name()),
            Self::inkVectorGraphicWidget(node) => Some(node.name()),
        }
    }
    pub fn as_compound(&self) -> Option<&dyn InkCompoundWidget> {
        match self {
            Self::inkCanvasWidget(node) => Some(node),
            Self::inkHorizontalPanelWidget(node) => Some(node),
            Self::inkVerticalPanelWidget(node) => Some(node),
            Self::inkScrollAreaWidget(node) => Some(node),
            Self::inkUniformGridWidget(node) => Some(node),
            Self::inkVirtualCompoundWidget(node) => Some(node),
            Self::inkFlexWidget(node) => Some(node),
            Self::inkCacheWidget(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_widget(&self) -> Option<&dyn InkWidget> {
        match self {
            Self::inkMultiChildren(_) => None,
            Self::inkCanvasWidget(widget) => Some(widget),
            Self::inkHorizontalPanelWidget(widget) => Some(widget),
            Self::inkVerticalPanelWidget(widget) => Some(widget),
            Self::inkScrollAreaWidget(widget) => Some(widget),
            Self::inkUniformGridWidget(widget) => Some(widget),
            Self::inkVirtualCompoundWidget(widget) => Some(widget),
            Self::inkFlexWidget(widget) => Some(widget),
            Self::inkCacheWidget(widget) => Some(widget),
            Self::inkTextWidget(widget) => Some(widget),
            Self::inkImageWidget(widget) => Some(widget),
            Self::inkVideoWidget(widget) => Some(widget),
            Self::inkMaskWidget(widget) => Some(widget),
            Self::inkBorderWidget(widget) => Some(widget),
            Self::inkShapeWidget(widget) => Some(widget),
            Self::inkCircleWidget(widget) => Some(widget),
            Self::inkRectangleWidget(widget) => Some(widget),
            Self::inkVectorGraphicWidget(widget) => Some(widget),
        }
    }
    pub fn as_leaf(&self) -> Option<&dyn InkLeafWidget> {
        match self {
            Self::inkTextWidget(widget) => Some(widget),
            Self::inkImageWidget(widget) => Some(widget),
            Self::inkVideoWidget(widget) => Some(widget),
            Self::inkMaskWidget(widget) => Some(widget),
            Self::inkBorderWidget(widget) => Some(widget),
            Self::inkShapeWidget(widget) => Some(widget),
            Self::inkCircleWidget(widget) => Some(widget),
            Self::inkRectangleWidget(widget) => Some(widget),
            Self::inkVectorGraphicWidget(widget) => Some(widget),
            _ => None,
        }
    }
}

pub trait WidgetTree {
    /// return the widget type
    fn get_widget_classname(&self, path: &[usize]) -> Option<String>;
    /// return the full path names to the widget
    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>>;
    /// return the full path indexes to the widget
    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>>;
}

pub trait ByIndex {
    /// find a widget by index
    fn by_index(&self, idx: usize) -> Option<Widget>;
}

pub trait ByName {
    /// find a widget by name
    fn by_name(&self, name: &str) -> Option<(usize, Widget)>;
}

pub trait Leaves {
    /// get widget summary for elements
    fn leaves(&self) -> Vec<WidgetSummary>;
}

impl<T> InkChildren for InkWrapper<T>
where
    T: InkChildren,
{
    fn orphans(&self) -> Vec<Widget> {
        self.data.orphans()
    }

    fn children(&self) -> Vec<InkWrapper<Widget>> {
        self.data.children()
    }
}

impl<T> ByIndex for T
where
    T: InkChildren,
{
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.orphans().get(idx).map(Clone::clone)
    }
}

impl<T> ByName for T
where
    T: InkChildren,
{
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        for (idx, child) in self.orphans().iter().enumerate() {
            if let Widget::inkMultiChildren(_) = &child {
                panic!("unexpected inkMultiChildren with name {name}");
            }
            if let Some(compound) = child.as_compound() {
                if compound.name() == name {
                    return Some((idx, child.clone()));
                }
            }
            continue;
        }
        None
    }
}

impl<T> Leaves for T
where
    T: InkCompoundWidget,
{
    fn leaves(&self) -> Vec<WidgetSummary> {
        let mut out = vec![];
        for child in self.children().iter() {
            if let Some(name) = child.data.name() {
                out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: name.to_string(),
                });
            }
        }
        out
    }
}

impl ByIndex for Widget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        match self {
            Self::inkMultiChildren(node) => node.by_index(idx),

            Self::inkCanvasWidget(node) => node.children.data.by_index(idx),
            Self::inkHorizontalPanelWidget(node) => node.children.data.by_index(idx),
            Self::inkVerticalPanelWidget(node) => node.children.data.by_index(idx),
            Self::inkScrollAreaWidget(node) => node.by_index(idx),
            Self::inkUniformGridWidget(node) => node.by_index(idx),
            Self::inkVirtualCompoundWidget(node) => node.by_index(idx),
            Self::inkFlexWidget(node) => node.by_index(idx),
            Self::inkCacheWidget(node) => node.by_index(idx),

            Self::inkTextWidget(leaf) => Some(Self::inkTextWidget(leaf.clone())),
            Self::inkImageWidget(leaf) => Some(Self::inkImageWidget(leaf.clone())),
            Self::inkVideoWidget(leaf) => Some(Self::inkVideoWidget(leaf.clone())),
            Self::inkMaskWidget(leaf) => Some(Self::inkMaskWidget(leaf.clone())),
            Self::inkBorderWidget(leaf) => Some(Self::inkBorderWidget(leaf.clone())),
            Self::inkShapeWidget(leaf) => Some(Self::inkShapeWidget(leaf.clone())),
            Self::inkCircleWidget(leaf) => Some(Self::inkCircleWidget(leaf.clone())),
            Self::inkRectangleWidget(leaf) => Some(Self::inkRectangleWidget(leaf.clone())),
            Self::inkVectorGraphicWidget(leaf) => Some(Self::inkVectorGraphicWidget(leaf.clone())),
        }
    }
}

impl WidgetTree for inkWidgetLibraryItemInstance {
    fn get_widget_classname(&self, path: &[usize]) -> Option<String> {
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));
        let last = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.as_ref().unwrap().by_index(*idx) {
                if let Widget::inkMultiChildren(_) = child {
                    panic!("encountered unexpected inkMultiChildren at index {idx}");
                }
                if child.as_compound().is_some() {
                    if i == last {
                        return Some(child.classname());
                    }
                    parent = Some(child.clone());
                    continue;
                }
                if child.as_leaf().is_some() {
                    return Some(child.classname());
                }
            }
        }
        None
    }

    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>> {
        let mut names: Vec<String> = vec![];
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));

        let depth = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.unwrap().by_index(*idx) {
                if let Some(name) = child.name() {
                    if child.as_compound().is_some() {
                        names.push(name.to_string());
                        parent = Some(child.clone());
                        continue;
                    }
                    if child.as_leaf().is_some() {
                        names.push(name.to_string());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                } else {
                    panic!("encountered unexpected inkMultiChildren at index {idx}");
                }
            }
            return None;
        }
        Some(names)
    }

    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>> {
        let mut indexes: Vec<usize> = vec![];
        let depth = path.len() - 1;
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));
        for (i, name) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }

            let found = match parent.unwrap() {
                Widget::inkCanvasWidget(node) => node.by_name(name),
                Widget::inkMultiChildren(node) => node.by_name(name),
                Widget::inkTextWidget(_)
                | Widget::inkImageWidget(_)
                | Widget::inkVideoWidget(_)
                | Widget::inkMaskWidget(_)
                | Widget::inkBorderWidget(_)
                | Widget::inkShapeWidget(_)
                | Widget::inkCircleWidget(_)
                | Widget::inkRectangleWidget(_)
                | Widget::inkVectorGraphicWidget(_) => {
                    if i < depth {
                        return None;
                    }
                    break;
                }
                Widget::inkHorizontalPanelWidget(node) => node.by_name(name),
                Widget::inkVerticalPanelWidget(node) => node.by_name(name),
                Widget::inkScrollAreaWidget(node) => node.by_name(name),
                Widget::inkUniformGridWidget(node) => node.by_name(name),
                Widget::inkVirtualCompoundWidget(node) => node.by_name(name),
                Widget::inkFlexWidget(node) => node.by_name(name),
                Widget::inkCacheWidget(node) => node.by_name(name),
            };
            if let Some((idx, widget)) = found {
                indexes.push(idx);
                parent = Some(widget);
                continue;
            }
            return None;
        }
        Some(indexes)
    }
}

impl inkWidgetLibraryResource {
    pub fn root(&self) -> &inkWidgetLibraryItem {
        self.library_items.get(0).expect("Root")
    }
    pub fn root_chunk(&self) -> &inkWidgetLibraryItemInstance {
        &self.root().package.file.data.root_chunk
    }
}

impl WidgetTree for inkWidgetLibraryResource {
    fn get_widget_classname(&self, path: &[usize]) -> Option<String> {
        self.root_chunk().get_widget_classname(path)
    }

    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>> {
        self.root_chunk().get_path_names(path)
    }

    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>> {
        self.root_chunk().get_path_indexes(path)
    }
}
