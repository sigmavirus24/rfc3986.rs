use std::collections::HashMap;

use uri::Uri;

/// The `UriBuilder` struct is used to construct instances of the `Uri` class.
///
/// # Examples
///
/// ```
/// use rfc3986::uri::Uri;
/// use rfc3986::uri_builder::UriBuilder;
/// let uri: Uri = UriBuilder::new()
///                     .add_scheme("https".to_string())
///                     .add_userinfo("username".to_string(), None)
///                     .add_host("example.com".to_string())
///                     .add_path("/login".to_string())
///                     .finalize();
/// assert_eq!(Some("https".to_string()), uri.scheme);
/// assert_eq!(Some("username".to_string()), uri.userinfo);
/// assert_eq!("example.com".to_string(), uri.host);
/// assert_eq!(Some("login".to_string()), uri.path);
/// ```
pub struct UriBuilder {
    scheme: Option<String>,
    userinfo: Option<String>,
    host: String,
    port: Option<u16>,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}


impl UriBuilder {
    /// Create a new UriBuilder struct with some default (empty) values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new().finalize();
    /// assert_eq!("".to_string(), uri.host);
    /// assert_eq!(None, uri.scheme);
    /// ```
    pub fn new() -> UriBuilder {
        UriBuilder {
            scheme: None,
            userinfo: None,
            host: "".to_string(),
            port: None,
            path: None,
            query: None,
            fragment: None
        }
    }

    /// Add a scheme to the Uri under construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_scheme("https".to_string())
    ///             .finalize();
    /// assert_eq!(Some("https".to_string()), uri.scheme);
    /// ```
    pub fn add_scheme(&mut self, scheme: String) -> &mut UriBuilder {
        self.scheme = Some(scheme);
        self
    }

    /// Add the user information to the Uri under construction.
    ///
    /// The username is required but the password argument is optional.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_userinfo("user".to_string(),
    ///                           Some("password".to_string()))
    ///             .finalize();
    /// assert_eq!(Some("user:password".to_string()), uri.userinfo);
    /// ```
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_userinfo("user".to_string(), None)
    ///             .finalize();
    /// assert_eq!(Some("user".to_string()), uri.userinfo);
    /// ```
    pub fn add_userinfo(&mut self,
                        username: String,
                        password: Option<String>) -> &mut UriBuilder {
        // TODO(sigmavirus24): Encode username and password
        self.userinfo = Some(match password {
            Some(password_str) => format!("{}:{}", username, password_str),
            None => username,
        });
        self
    }

    /// Add the host to the Uri under construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_host("example.com".to_string())
    ///             .finalize();
    /// assert_eq!("example.com", uri.host);
    /// ```
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_host("127.0.0.1".to_string())
    ///             .finalize();
    /// assert_eq!("127.0.0.1".to_string(), uri.host);
    /// ```
    pub fn add_host(&mut self, host: String) -> &mut UriBuilder {
        // TODO(sigmavirus24): Verify the host is valid
        self.host = host;
        self
    }

    /// Add the port number to the Uri under construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_port(80)
    ///             .finalize();
    /// assert_eq!(Some(80), uri.port);
    /// ```
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_port(443)
    ///             .finalize();
    /// assert_eq!(Some(443), uri.port);
    /// ```
    pub fn add_port(&mut self, port: u16) -> &mut UriBuilder {
        self.port = Some(port);
        self
    }

    pub fn add_path(&mut self, path: String) -> &mut UriBuilder {
        self.path = Some(match path.starts_with('/') {
            true => path[1..].to_string(),
            false => path,
        });
        self
    }

    /// Add a query string to the Uri under construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_query_string("a=1&b=2".to_string())
    ///             .finalize();
    /// assert_eq!(Some("a=1&b=2".to_string()), uri.query);
    /// ```
    pub fn add_query_string(&mut self, query: String) -> &mut UriBuilder {
        self.query = Some(query);
        self
    }

    pub fn add_query_map(&mut self,
                         query_map: &HashMap<String, String>) -> &mut UriBuilder {
        self
    }

    /// Finalize the `UriBuilder` and create a `Uri` from it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfc3986::uri_builder::UriBuilder;
    /// let uri = UriBuilder::new()
    ///             .add_host("example.com".to_string())
    ///             .finalize();
    /// assert_eq!("example.com".to_string(), uri.generate_authority());
    /// ```
    pub fn finalize(&self) -> Uri {
        Uri {
            scheme: self.scheme.clone(),
            userinfo: self.userinfo.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            path: self.path.clone(),
            query: self.query.clone(),
            fragment: self.fragment.clone(),
        }
    }
}
