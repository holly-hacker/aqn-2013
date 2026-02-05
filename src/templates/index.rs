use askama::Template;

use crate::data::{DatabaseData, Forum};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub forums: Vec<&'a Forum>,
}

impl<'a> From<&'a DatabaseData> for IndexTemplate<'a> {
    fn from(value: &'a DatabaseData) -> Self {
        Self {
            forums: {
                let mut values = value.forums.values().collect::<Vec<_>>();
                values.sort_by_key(|x| x.display_order);
                values
            },
        }
    }
}
