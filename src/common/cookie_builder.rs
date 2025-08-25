use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use poem::web::cookie::Cookie;
use std::ops::Add;
use std::time::Duration;

pub struct CookieBuilder {
    cookie: Cookie,
}

impl CookieBuilder {
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.cookie.set_domain(domain);
        self
    }

    pub fn expires(mut self, expires: DateTime<impl TimeZone>) -> Self {
        self.cookie.set_expires(expires);
        self
    }

    pub fn expires_by_delta(self, time_delta: TimeDelta) -> Self {
        self.expires(Utc::now().add(time_delta))
    }

    pub fn http_only(mut self) -> Self {
        self.cookie.set_http_only(true);
        self
    }

    /// Not recommended, use `expires_by_delta` instead.
    pub fn max_age(mut self, value: Duration) -> Self {
        self.cookie.set_max_age(value);
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.cookie.set_path(path);
        self
    }

    pub fn same_site(mut self, same_site: poem::web::cookie::SameSite) -> Self {
        self.cookie.set_same_site(same_site);
        self
    }

    pub fn secure(mut self) -> Self {
        self.cookie.set_secure(true);
        self
    }

    pub fn partitioned(mut self) -> Self {
        self.cookie.set_partitioned(true);
        self
    }

    pub fn build(self) -> Cookie {
        self.cookie
    }
}

pub trait CookieBuilderExt {
    fn into_builder(self) -> CookieBuilder;
}

impl CookieBuilderExt for Cookie {
    fn into_builder(self) -> CookieBuilder {
        CookieBuilder { cookie: self }
    }
}
