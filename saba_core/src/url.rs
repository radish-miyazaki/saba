use alloc::{
    string::{String, ToString},
    vec::Vec,
};

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

    pub fn parse(&mut self) -> Result<Url, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }

        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            // ポート番号が指定されている場合は、その手前までをホスト名として返す
            url_parts[0][..index].to_string()
        } else {
            // ポート番号が指定されていない場合は、そのままホスト名として返す
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            // ポート番号が指定されていない場合はデフォルトの80番ポートを返す
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_paths: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_paths.len() < 2 {
            // パスが存在しないケース
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_paths[1].splitn(2, '?').collect();

        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_paths: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_paths.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_paths[1].splitn(2, '?').collect();

        if path_and_searchpart.len() < 2 {
            // パスが存在しないケース
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();

        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8080".to_string();

        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8080/index.html".to_string();

        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();

        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path_searchpart() {
        let url = "http://example.com:8080/index.html?a=123&b=456".to_string();

        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();

        let expected = Err("Only HTTP scheme is supported".to_string());

        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_schema() {
        let url = "https://example.com:8080/index.html".to_string();

        let expected = Err("Only HTTP scheme is supported".to_string());

        assert_eq!(expected, Url::new(url).parse());
    }
}
