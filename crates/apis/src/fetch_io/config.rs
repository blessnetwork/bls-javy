use crate::APIConfig;

// Use crate visibility to avoid exposing the property outside the crate
#[derive(Debug)]
pub(crate) struct FetchIOConfig {
    pub(super) prefix: String,
}

// Always have a default value for every config.
impl Default for FetchIOConfig {
    fn default() -> Self {
        Self {
            prefix: "Default prefix: ".to_string(),
        }
    }
}

// Define one or more methods on `APIConfig`, not `FetchIOConfig`, to set properties.
impl APIConfig {
    /// Sets the prefix for `Javy.Env.print`.
    pub fn prefix(&mut self, prefix: String) -> &mut Self {
        self.fetch_io.prefix = prefix;
        self
    }
}