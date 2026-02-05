use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::db::generated::{mybb_forums::Model as MybbForum, mybb_users::Model as MybbUser};

#[derive(Serialize, Deserialize)]
pub struct DatabaseData {
    pub users: BTreeMap<u32, User>,
    pub forums: BTreeMap<u16, Forum>,
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

#[derive(Serialize, Deserialize)]
pub struct Forum {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub parent: Option<u16>,
    pub display_order: u16,
}

impl TryFrom<MybbForum> for Forum {
    type Error = anyhow::Error;

    fn try_from(value: MybbForum) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.fid,
            name: value.name,
            description: value.description,
            parent: {
                let parent = value.parentlist.split(',').rev().nth(1);
                if let Some(parent) = parent {
                    Some(parent.parse()?)
                } else {
                    None
                }
            },
            display_order: value.disporder,
        })
    }
}
