use std::ascii::AsciiExt;
use std::string::String;

/// The container for our parsed Uri.
/// 
/// Per RFC 3986, there are five parts to a Uri:
///
/// 1. Scheme, e.g., http://, https://, etc.
/// 1. Authority which is composed of userinfo@host:port, e.g.,
///    user:pass@example.com:80, example.com:22, example.com, etc.
/// 1. Path, e.g., /index.php
/// 1. Query, e.g., ?lang=en
/// 1. Fragment, e.g., #anchor
///
/// The Uri struct in this library, however, parses the authority into its
/// components and provides a method to re-generate it.
///
/// # Examples
/// let uri = Uri::from_str("https://github.com/rust-lang/rust");
/// assert_eq!("github.com", uri.host)
#[derive(Debug)]
pub struct Uri {
    pub scheme: Option<String>,
    pub userinfo: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Uri {
    /// The `generate_authority` method will generate and return the
    /// authority for a parsed URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri::Uri;
    /// let uri: Uri = Uri::from_str("https://github.com/rust-lang/rust");
    /// assert_eq!("github.com", uri.generate_authority());
    /// ```
    ///
    /// ```
    /// use rfc3986::uri::Uri;
    /// let uri: Uri = Uri::from_str("https://username:password@github.com/rust-lang/rust");
    /// assert_eq!("username:password@github.com", uri.generate_authority());
    /// ```
    ///
    /// ```
    /// use rfc3986::uri::Uri;
    /// let uri: Uri = Uri::from_str("https://user:pass@example.com:444/");
    /// assert_eq!("user:pass@example.com:444", uri.generate_authority());
    /// ```
    pub fn generate_authority(&self) -> String {
        let mut authority = String::new();

        if let Some(ref userinfo) = self.userinfo {
            authority.push_str(&userinfo);
            authority.push_str("@");
        }

        authority.push_str(&self.host);

        if let Some(port) = self.port {
            let port_string = format!("{}", port);
            authority.push_str(":");
            authority.push_str(port_string.as_str());
        }

        authority
    }

    /// The `from_str` function will parse a `str` into a `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri::Uri;
    /// let uri: Uri = Uri::from_str("https://github.com/rust-lang/rust");
    /// ```
    pub fn from_str(uri: &str) -> Uri {
        let scheme: Option<String>;
        let userinfo: Option<String>;
        let host: String;
        let port: Option<u16>;
        let query: Option<String>;
        let fragment: Option<String>;
        let mut rest: &str;

        if uri.contains("://") {
            let parts: Vec<&str> = uri.splitn(2, "://").collect();
            scheme = Some(parts[0].to_string());
            rest = parts[1];
        } else {
            scheme = None;
            rest = uri;
        }

        // Handle the case where a Uri starts with // but doesn't have an
        // explicit `scheme:`
        if scheme == None && rest.starts_with("//") {
            rest = &rest[2..];
        }

        // Find where the user information ends (the first @)
        if rest.contains('@') {
            let parts: Vec<&str> = rest.splitn(2, '@').collect();
            userinfo = Some(parts[0].to_string());
            rest = parts[1];
        } else {
            userinfo = None;
        }

        // Find the port and parse it out along with the host
        if rest.contains(':') {
            let parts: Vec<&str> = rest.splitn(2, ':').collect();
            host = parts[0].to_string();
            let other_parts: Vec<&str> = parts[1].splitn(2, '/').collect();
            port = Some(other_parts[0].parse::<u16>().unwrap());
            rest = other_parts[1];
        } else {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            host = parts[0].to_string();
            rest = parts[1];
            port = None;
        }
        
        if rest.len() >= 1 {
            // Now working backwards, find the fragment (if it exists)
            if rest.contains('#') {
                // NOTE(sigmavirus24): rsplitn reverses the order of the
                // parts
                let parts: Vec<&str> = rest.rsplitn(2, '#').collect();
                fragment = Some(parts[0].to_string());
                rest = parts[1];
            } else {
                fragment = None;
            }

            // Now that we've parsed out the fragment, let's find the query
            if rest.contains('?') {
                let parts: Vec<&str> = rest.rsplitn(2, '?').collect();
                query = Some(parts[0].to_string());
                rest = parts[1];
            } else {
                query = None;
            }
        } else {
            fragment = None;
            query = None;
        }

        // Finally, if there's anything left, it's probably the path
        let path: Option<String> = if rest.len() < 1 {
            None
        } else {
            Some(rest.to_string())
        };
        Uri {
            scheme: scheme,
            userinfo: userinfo,
            host: host,
            port: port,
            path: path,
            query: query,
            fragment: fragment,
        }
    }

    fn validate_scheme(&self) -> &Uri {
        if let Some(ref scheme) = self.scheme {
            for character in scheme.chars() {
                if !(character.is_ascii() && character.is_alphabetic()) {
                    panic!("'{}' is not valid in a URI scheme", character);
                }
            }
        }
        self
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Uri) -> bool {
        (self.scheme == other.scheme &&
         self.userinfo == other.userinfo &&
         self.host == other.host &&
         self.port == other.port &&
         self.path == other.path &&
         self.query == other.query &&
         self.fragment == other.fragment)
    }
}


#[cfg(test)]
mod tests {
    use super::Uri;

    fn assert_parses(url: &str, into: &Uri) {
        let parsed = &Uri::from_str(url);
        assert_eq!(into, parsed);
    }

    #[test]
    fn it_parses_a_simple_url() {
        let url: &str = "https://github.com/sigmavirus24";
        assert_parses(url, &Uri {
            scheme: Some("https".to_string()),
            userinfo: None,
            host: "github.com".to_string(),
            port: None,
            path: Some("sigmavirus24".to_string()),
            query: None,
            fragment: None,
        });
    }

    #[test]
    fn it_parses_a_schemeless_url() {
        let url: &str = "github.com/sigmavirus24";
        assert_parses(url, &Uri {
            scheme: None,
            userinfo: None,
            host: "github.com".to_string(),
            port: None,
            path: Some("sigmavirus24".to_string()),
            query: None,
            fragment: None,
        });
    }

    #[test]
    fn it_parses_a_scheme_relative_url() {
        let url: &str = "//github.com/sigmavirus24";
        assert_parses(url, &Uri {
            scheme: None,
            userinfo: None,
            host: "github.com".to_string(),
            port: None,
            path: Some("sigmavirus24".to_string()),
            query: None,
            fragment: None,
        });
    }

    #[test]
    #[should_panic]
    fn it_validates_a_scheme() {
        let uri = Uri::from_str("h0tps://github.com");
        uri.validate_scheme();
    }

}
