use super::{
    inkCanvasWidget, inkMultiChildren, inkWidgetLibraryItem, inkWidgetLibraryItemInstance,
    inkWidgetLibraryResource, Widget, WidgetSummary,
};

pub trait WidgetTree {
    fn get_widget_kind(&self, path: &[usize]) -> Option<String>;
    fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>>;
}

pub trait ByIndex {
    fn by_index(&self, idx: usize) -> Option<Widget>;
}

pub trait Leaves {
    fn leaves(&self) -> Vec<WidgetSummary>;
}

impl ByIndex for inkCanvasWidget {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children.data.by_index(idx)
    }
}

impl ByIndex for inkMultiChildren {
    fn by_index(&self, idx: usize) -> Option<Widget> {
        self.children.get(idx).map(|child| child.data.clone())
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
                    Widget::inkMultiChildren(_)
                    | Widget::inkHorizontalPanelWidget(_)
                    | Widget::inkVerticalPanelWidget(_) => {
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
}
