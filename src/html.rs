use askama::Template;

use crate::model::Url;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "rank.html")]
pub struct RankTemplate {
    pub urls: Vec<Url>,
    pub short_url_domain: String,
}

#[derive(Template)]
#[template(path = "msg.html")]
pub struct MsgTemplate {
    pub is_ok: bool,
    pub msg: String,
    pub target_url: Option<String>,
}

impl MsgTemplate {
    fn new(is_ok: bool, msg: String, target_url: Option<String>) -> Self {
        Self {
            is_ok,
            msg,
            target_url,
        }
    }
    pub fn ok(msg: &str, target_url: &str) -> Self {
        Self::new(true, msg.to_string(), Some(target_url.to_string()))
    }
    pub fn err(msg: &str) -> Self {
        Self::new(false, msg.to_string(), None)
    }
    pub fn target_url(&self) -> String {
        match self.target_url.clone() {
            Some(target_url) => target_url,
            None => format!("javascript:history.back(-1)"),
        }
    }
}

impl Default for MsgTemplate {
    fn default() -> Self {
        Self::new(false, String::from(""), None)
    }
}
