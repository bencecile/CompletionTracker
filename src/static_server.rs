use rouille::{Response};

static BUNDLE_JS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bundle.js"));
static MAIN_HTML: &'static str = include_str!(concat!(env!("OUT_DIR"), "/main.html"));

/// Give a special debug implementation of the static server to make it cache-less
pub struct StaticServer;
impl StaticServer {
    pub fn serve_main_html(&self) -> Response { Response::html(MAIN_HTML) }
    pub fn serve_bundle_js(&self) -> Response { Response::from_data("text/javascript", BUNDLE_JS) }
}
