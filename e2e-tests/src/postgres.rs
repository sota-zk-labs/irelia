use std::borrow::Cow;
use std::collections::HashMap;

use testcontainers_modules::testcontainers::core::WaitFor;
use testcontainers_modules::testcontainers::{CopyToContainer, Image};

const NAME: &str = "postgres";
const TAG: &str = "latest";

pub struct Postgres {
    env_vars: HashMap<String, String>,
    copy_to_sources: Vec<CopyToContainer>,
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_USER".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "postgres".to_owned());
        Self {
            env_vars,
            copy_to_sources: Vec::new(),
        }
    }
}

impl Image for Postgres {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![
            WaitFor::message_on_stderr("database system is ready to accept connections"),
            WaitFor::message_on_stdout("database system is ready to accept connections"),
        ]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        &self.env_vars
    }

    fn copy_to_sources(&self) -> impl IntoIterator<Item = &CopyToContainer> {
        &self.copy_to_sources
    }
}
