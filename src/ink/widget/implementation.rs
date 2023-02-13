use super::{inkCanvasWidget, inkMultiChildren, Widget, WidgetSummary};

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
            Widget::inkCanvasWidget(node) => node.children.data.by_index(idx),
            Widget::inkMultiChildren(node) => node.by_index(idx),
            Widget::inkTextWidget(leaf) => Some(Widget::inkTextWidget(leaf.clone())),
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
