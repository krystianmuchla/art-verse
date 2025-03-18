use crate::canvas::canvas::Canvas;
use crate::tool::tool_bar::ToolBar;
use web_sys::{window, Document, HtmlElement, Window};

pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub tool_bar: ToolBar,
    pub canvas: Canvas,
}

impl Dom {
    pub fn new() -> Dom {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let canvas = Canvas::new(&document);
        let tool_bar = ToolBar::new(&document);
        Dom {
            window,
            document,
            body,
            tool_bar,
            canvas,
        }
    }
}
