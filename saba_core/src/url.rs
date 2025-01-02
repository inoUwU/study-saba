use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    /// URLを取得します
    pub fn host(&self) -> String {
        self.host.clone()
    }

    /// ポート番号を取得します
    pub fn port(&self) -> String {
        self.port.clone()
    }

    /// パス部分を取得します
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// クエリ部分を取得します
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    /// コンストラクタ
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: String::new(),
            port: String::new(),
            path: String::new(),
            searchpart: String::new(),
        }
    }

    /// スキームがHTTPかどうかを判定します
    pub fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    /// URLからホスト部分を抽出します
    fn extract_host(&self) -> String {
        // http://example.com:8080/path/to/resource?query=1
        let url_parts: Vec<&str> = self.url.trim_start_matches("http://").split('/').collect();
        if let Some(index) = url_parts[0].find(':') {
            // ポート番号が含まれている場合
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    /// ポート番号を抽出します
    fn extract_port(&self) -> String {
        // http://example.com:8080/path/to/resource?query=1
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            // ポート番号が含まれている場合
            url_parts[0][index + 1..].to_string()
        } else {
            // デフォルトのポート番号を返す
            "80".to_string()
        }
    }

    /// パス部分を抽出します
    fn extract_path(&self) -> String {
        // http://example.com:8080/path/to/resource?query=1
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 分割数を指定
            .collect();

        // パス部分が存在しない場合は空文字を返す
        if url_parts.len() < 2 {
            return String::new();
        }

        // パスが存在する場合はパス部分を返す
        // ?部分で2つに分割する
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        path_and_searchpart[0].to_string()
    }

    /// クエリ部分を抽出します
    fn extract_searchpart(&self) -> String {
        // http://example.com:8080/path/to/resource?query=1
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 分割数を指定
            .collect();

        // パス部分が存在しない場合は空文字を返す
        if url_parts.len() < 2 {
            return String::new();
        }

        // パスが存在する場合はパス部分を返す
        // ?部分で2つに分割する
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        if path_and_searchpart.len() < 2 {
            return String::new();
        }
        path_and_searchpart[1].to_string()
    }

    /// URLを解析します
    pub fn parse(&mut self) -> Result<Self, String> {
        // RFC1738にスキームの省略は定義されていないので今回はエラーを返す
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }
        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URLのホスト部分を取得できるか
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

    /// URLにポート番号が含まれている場合、ポート番号を取得できるか
    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    /// URLにパス部分が含まれている場合、パス部分を取得できるか
    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    /// URLにポート番号が含まれていない場合、デフォルトのポート番号を取得できるか
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

    /// URLにクエリ部分が含まれている場合、クエリ部分を取得できるか
    #[test]
    fn test_url_host_port_path_searchpart() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }

    // ================================== 失敗するテストケース ==================================

    /// スキームがHTTP以外の場合、エラーを返すか
    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported".to_string());

        assert_eq!(expected, Url::new(url).parse());
    }

    /// スキームがHTTPSの場合、エラーを返すか
    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com".to_string();
        let expected = Err("Only HTTP scheme is supported".to_string());

        assert_eq!(expected, Url::new(url).parse());
    }
}
