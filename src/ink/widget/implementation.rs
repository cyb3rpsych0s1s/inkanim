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
            fn children(&self) -> Vec<Widget> {
                self.children.data.children()
            }

            fn nodes(&self) -> Vec<InkWrapper<Widget>> {
                self.children.data.nodes()
            }
        }
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
    fn children(&self) -> Vec<Widget>;
    fn nodes(&self) -> Vec<InkWrapper<Widget>>;
}

pub trait InkCompoundWidget: InkWidget + InkChildren {}

impl InkChildren for inkMultiChildren {
    fn children(&self) -> Vec<Widget> {
        self.children.iter().map(|x| x.data.clone()).collect()
    }

    fn nodes(&self) -> Vec<InkWrapper<Widget>> {
        self.children.iter().map(|x| x.clone()).collect()
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
            Widget::inkMultiChildren(widget) => widget.classname(),

            Widget::inkCanvasWidget(widget) => widget.classname(),
            Widget::inkHorizontalPanelWidget(widget) => widget.classname(),
            Widget::inkVerticalPanelWidget(widget) => widget.classname(),
            Widget::inkScrollAreaWidget(widget) => widget.classname(),
            Widget::inkUniformGridWidget(widget) => widget.classname(),
            Widget::inkVirtualCompoundWidget(widget) => widget.classname(),
            Widget::inkFlexWidget(widget) => widget.classname(),
            Widget::inkCacheWidget(widget) => widget.classname(),

            Widget::inkTextWidget(widget) => widget.classname(),
            Widget::inkImageWidget(widget) => widget.classname(),
            Widget::inkVideoWidget(widget) => widget.classname(),
            Widget::inkMaskWidget(widget) => widget.classname(),
            Widget::inkBorderWidget(widget) => widget.classname(),
            Widget::inkShapeWidget(widget) => widget.classname(),
            Widget::inkCircleWidget(widget) => widget.classname(),
            Widget::inkRectangleWidget(widget) => widget.classname(),
            Widget::inkVectorGraphicWidget(widget) => widget.classname(),
        }
    }
}

pub trait WidgetTree {
    /// return the widget type
    fn get_widget_kind(&self, path: &[usize]) -> Option<String>;
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

impl<T> ByIndex for T
where
    T: InkChildren,
{
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children().get(idx).map(Clone::clone)
    }
}

impl<T> ByName for T
where
    T: InkChildren,
{
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        for (idx, child) in self.children().iter().enumerate() {
            match &child {
                Widget::inkMultiChildren(_) => {
                    panic!("unexpected inkMultiChildren with name {name}")
                }
                Widget::inkCanvasWidget(node) if node.name == name => {
                    return Some((idx, child.clone()))
                }
                Widget::inkTextWidget(node) if node.name == name => {
                    return Some((idx, child.clone()))
                }
                Widget::inkHorizontalPanelWidget(node) if node.name == name => {
                    return Some((idx, child.clone()))
                }
                Widget::inkVerticalPanelWidget(node) if node.name == name => {
                    return Some((idx, child.clone()))
                }
                _ => continue,
            };
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
        for child in self.nodes().iter() {
            match child.data {
                Widget::inkTextWidget(ref leaf) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: leaf.name.clone(),
                }),
                Widget::inkCanvasWidget(ref node) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: node.name.clone(),
                }),
                Widget::inkHorizontalPanelWidget(ref node) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: node.name.clone(),
                }),
                Widget::inkVerticalPanelWidget(ref node) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: node.name.clone(),
                }),
                _ => {}
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
    fn get_widget_kind(&self, path: &[usize]) -> Option<String> {
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));
        let last = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.as_ref().unwrap().by_index(*idx) {
                match child {
                    Widget::inkCanvasWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkCanvasWidget(node.clone()));
                        continue;
                    }
                    Widget::inkHorizontalPanelWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkHorizontalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVerticalPanelWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkVerticalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkMultiChildren(_node) => {
                        panic!("encountered unexpected inkMultiChildren at index {idx}");
                    }
                    Widget::inkTextWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkImageWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkVideoWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkMaskWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkBorderWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkShapeWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkCircleWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkRectangleWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkVectorGraphicWidget(leaf) => return Some(leaf.classname()),
                    Widget::inkScrollAreaWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkScrollAreaWidget(node.clone()));
                        continue;
                    }
                    Widget::inkUniformGridWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkUniformGridWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVirtualCompoundWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkVirtualCompoundWidget(node.clone()));
                        continue;
                    }
                    Widget::inkFlexWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkFlexWidget(node.clone()));
                        continue;
                    }
                    Widget::inkCacheWidget(node) => {
                        if i == last {
                            return Some(node.classname());
                        }
                        parent = Some(Widget::inkCacheWidget(node.clone()));
                        continue;
                    }
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
                match child {
                    Widget::inkCanvasWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkCanvasWidget(node.clone()));
                        continue;
                    }
                    Widget::inkHorizontalPanelWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkHorizontalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVerticalPanelWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkVerticalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkMultiChildren(_) => {
                        panic!("encountered unexpected inkMultiChildren at index {idx}");
                    }
                    Widget::inkTextWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkImageWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkVideoWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }

                    Widget::inkMaskWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkBorderWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkShapeWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }

                    Widget::inkCircleWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkRectangleWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkVectorGraphicWidget(leaf) => {
                        names.push(leaf.name.clone());
                        if i < depth {
                            return None;
                        }
                        break;
                    }
                    Widget::inkScrollAreaWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkScrollAreaWidget(node.clone()));
                        continue;
                    }
                    Widget::inkUniformGridWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkUniformGridWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVirtualCompoundWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkVirtualCompoundWidget(node.clone()));
                        continue;
                    }
                    Widget::inkFlexWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkFlexWidget(node.clone()));
                        continue;
                    }
                    Widget::inkCacheWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkCacheWidget(node.clone()));
                        continue;
                    }
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
    fn get_widget_kind(&self, path: &[usize]) -> Option<String> {
        self.root_chunk().get_widget_kind(path)
    }

    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>> {
        self.root_chunk().get_path_names(path)
    }

    fn get_path_indexes(&self, path: &[&str]) -> Option<Vec<usize>> {
        self.root_chunk().get_path_indexes(path)
    }
}
