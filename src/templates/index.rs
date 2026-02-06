use askama::Template;

use crate::{
    data::{DatabaseData, Forum},
    templates::BaseProps,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub base_props: &'a BaseProps,

    pub forums: Vec<&'a Forum>,
}

impl<'a> From<(&'a DatabaseData, &'a BaseProps)> for IndexTemplate<'a> {
    fn from((db, base_props): (&'a DatabaseData, &'a BaseProps)) -> Self {
        Self {
            base_props,
            forums: {
                let mut values = db.forums.values().collect::<Vec<_>>();
                values.sort_by_key(|x| x.display_order);
                values
            },
        }
    }
}
