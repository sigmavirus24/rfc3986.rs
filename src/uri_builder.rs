pub struct UriBuilder {
    scheme: Option<&str>,
    userinfo: Option<&str>,
    host: &str,
    port: Option<u16>,
    path: Option<&str>,
    query: Option<&str>,
    fragment: Option<&str>,
}


impl UriBuilder {
    pub fn new() -> UriBuilder {
        UriBuilder {
            scheme: None,
            userinfo: None,
            host: "",
            port: None,
            path: None,
            query: None,
            fragment: None
        }
    }

    pub fn add_scheme(&mut self, scheme: &str) -> &mut UriBuilder {
        self.scheme = Some(scheme);
        self
    }

    pub fn add_userinfo(&mut self,
                        username: &str,
                        password: &str) -> &mut UriBuilder {
        // TODO(sigmavirus24): Encode username and password
        self.userinfo = &format!("{}:{}", username, password);
        self
    }
}
