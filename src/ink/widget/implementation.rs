use super::{
    inkCanvasWidget, inkHorizontalPanelWidget, inkMultiChildren, inkVerticalPanelWidget,
    inkWidgetLibraryItem, inkWidgetLibraryItemInstance, inkWidgetLibraryResource, Widget,
    WidgetSummary,
};

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

impl ByIndex for inkMultiChildren {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children.get(idx).map(|child| child.data.clone())
    }
}

impl ByName for inkMultiChildren {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        for (idx, child) in self.children.iter().enumerate() {
            match &child.data {
                Widget::inkMultiChildren(_) => {
                    panic!("unexpected inkMultiChildren with name {name}")
                }
                Widget::inkCanvasWidget(node) if node.name == name => {
                    return Some((idx, child.data.clone()))
                }
                Widget::inkTextWidget(node) if node.name == name => {
                    return Some((idx, child.data.clone()))
                }
                Widget::inkHorizontalPanelWidget(node) if node.name == name => {
                    return Some((idx, child.data.clone()))
                }
                Widget::inkVerticalPanelWidget(node) if node.name == name => {
                    return Some((idx, child.data.clone()))
                }
                _ => continue,
            };
        }
        None
    }
}

impl ByIndex for inkCanvasWidget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children.data.by_index(idx)
    }
}

impl ByName for inkCanvasWidget {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        self.children.data.by_name(name)
    }
}

impl ByName for inkHorizontalPanelWidget {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        self.children.data.by_name(name)
    }
}

impl ByName for inkVerticalPanelWidget {
    fn by_name(&self, name: &str) -> Option<(usize, Widget)> {
        self.children.data.by_name(name)
    }
}

impl ByIndex for Widget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        match self {
            Self::inkCanvasWidget(node) => node.children.data.by_index(idx),
            Self::inkHorizontalPanelWidget(node) => node.children.data.by_index(idx),
            Self::inkVerticalPanelWidget(node) => node.children.data.by_index(idx),
            Self::inkMultiChildren(node) => node.by_index(idx),
            Self::inkTextWidget(leaf) => Some(Self::inkTextWidget(leaf.clone())),
        }
    }
}

impl Leaves for inkMultiChildren {
    fn leaves(&self) -> Vec<WidgetSummary> {
        let mut out = vec![];
        for child in self.children.iter() {
            match child.data {
                Widget::inkTextWidget(ref leaf) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: leaf.name.clone(),
                }),
                Widget::inkCanvasWidget(ref node) => out.push(WidgetSummary {
                    HandleId: child.handle_id,
                    Name: node.name.clone(),
                }),
                _ => {}
            }
        }
        out
    }
}

impl Leaves for inkCanvasWidget {
    fn leaves(&self) -> Vec<WidgetSummary> {
        self.children.data.leaves()
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
                            return Some("inkCanvasWidget".to_string());
                        }
                        parent = Some(Widget::inkCanvasWidget(node.clone()));
                        continue;
                    }
                    Widget::inkHorizontalPanelWidget(node) => {
                        if i == last {
                            return Some("inkHorizontalPanelWidget".to_string());
                        }
                        parent = Some(Widget::inkHorizontalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVerticalPanelWidget(node) => {
                        if i == last {
                            return Some("inkVerticalPanelWidget".to_string());
                        }
                        parent = Some(Widget::inkVerticalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkMultiChildren(_node) => {
                        panic!("encountered unexpected inkMultiChildren at index {idx}");
                    }
                    Widget::inkTextWidget(_leaf) => return Some("inkTextWidget".to_string()),
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
                Widget::inkTextWidget(_) => {
                    if i < depth {
                        return None;
                    }
                    break;
                }
                Widget::inkHorizontalPanelWidget(node) => node.by_name(name),
                Widget::inkVerticalPanelWidget(node) => node.by_name(name),
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
