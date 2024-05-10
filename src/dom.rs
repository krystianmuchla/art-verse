use web_sys::{Document, HtmlElement, Window};

pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
}

impl Dom {
    pub fn new(window: Window, document: Document, body: HtmlElement) -> Dom {
        Dom {
            window,
            document,
            body,
        }
    }
}
