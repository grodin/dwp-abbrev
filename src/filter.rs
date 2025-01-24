use crate::abbreviation::Entry;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Filter {
    query: String,
}

impl Filter {
    pub fn match_against<'a>(&self, entry: &'a Entry) -> Option<&'a Entry> {
        // If this filter has no value, everything matches
        if self.query.is_empty() {
            return Some(entry);
        }

        let mut alpha_num_only: String = entry.abbreviation().to_string().to_ascii_uppercase();
        alpha_num_only.retain(|c| c.is_ascii_alphanumeric());

        let alpha_num_no_connectives = CONNECTIVES
            .into_iter()
            .fold(alpha_num_only.clone(), |s, connective| {
                s.replace(connective, "")
            });

        fn matches(filter: &str, s: &str) -> bool {
            s.trim().starts_with(filter.trim())
        }

        if matches(&self.query, &alpha_num_only) || matches(&self.query, &alpha_num_no_connectives)
        {
            return Some(entry);
        }
        None
    }
}

impl From<&str> for Filter {
    fn from(value: &str) -> Self {
        let mut query = value.trim_start().to_ascii_uppercase();
        query.retain(|c| c.is_ascii_alphanumeric());
        Filter { query }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query)
    }
}

const CONNECTIVES: [&str; 3] = [" and ", " to ", " or "];
