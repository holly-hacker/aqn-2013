use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::db::generated::mybb_users::Model as MybbUser;

#[derive(Serialize, Deserialize)]
pub struct DatabaseData {
    pub users: BTreeMap<u32, User>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub usergroups: Vec<u16>,
}

impl TryFrom<MybbUser> for User {
    type Error = anyhow::Error;

    fn try_from(value: MybbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.uid,
            username: value.username,
            usergroups: {
                let mut vec = Vec::new();
                vec.push(value.usergroup);
                for item in value.additionalgroups.split(',') {
                    if !item.is_empty() {
                        vec.push(item.parse().expect("parse usergroup"))
                    }
                }
                vec
            },
        })
    }
}
