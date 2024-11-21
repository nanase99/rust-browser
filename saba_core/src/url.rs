use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }
}

impl Url {
    pub fn parse(&mut self) -> Result<Self, String> {
        if self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }
        unimplemented!();
    }
}

impl Url {
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }
}
