use scraper::{ElementRef, Html, Selector};

/// Website content deserializer.
pub struct Deserializer {
    /// The internal html page.
    /// Actually not *that* internal.
    page_html: String,

    /// The parsed HTML page.
    dom: Html
}

impl Deserializer {
    /// Constructs the Deserializer from the provided HTML document.
    pub fn from_html(html: String) -> Self {
        let parsed = Html::parse_document(html.as_str());

        Self {
            page_html: html,
            dom: parsed
        }
    }

    /// Get the raw HTML of this deserializer.
    pub fn get_raw_html(&self) -> String {
        self.page_html.to_owned()
    }

    /// Search for element(s) matching the CSS selector.
    pub fn search(&self, selector: &str) -> Vec<ElementRef<'_>> {
        let dom_selector = Selector::parse(selector).unwrap();

        self.dom.select(&dom_selector).collect::<Vec<_>>()
    }
}