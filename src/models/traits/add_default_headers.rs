//! Trait to be implemnted by [`tide::ResponseBuilder`] to add default headers.
//!

pub trait AddDefaultHeaders {
    /// Add default headers to a [`tide::ResponseBuilder`] to be included to the
    /// eventual [`tide::Response`].
    fn add_default_headers(self) -> Self;
}

impl AddDefaultHeaders for tide::ResponseBuilder {
    /// Add default headers to a [`tide::ResponseBuilder`] to be included to the
    /// eventual [`tide::Response`].
    ///
    /// This implementation is usage specific, and is not intended to be used
    /// outside of this crate.
    ///
    /// It adds the following headers:
    /// - [`http_types::headers::ACCESS_CONTROL_ALLOW_ORIGIN`] as `*`
    /// - [`http_types::headers::ACCESS_CONTROL_ALLOW_METHODS`] as `GET,DELETE,PATCH,POST,PUT`
    /// - [`http_types::mime::JSON`] as the content type
    fn add_default_headers(self) -> Self {
        self.header(http_types::headers::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(
                http_types::headers::ACCESS_CONTROL_ALLOW_METHODS,
                "GET,DELETE,PATCH,POST,PUT",
            )
            .content_type(http_types::mime::JSON)
    }
}
