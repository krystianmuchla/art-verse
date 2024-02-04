use web_sys::{Document, HtmlElement, Window};

pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
}
