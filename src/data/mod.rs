use std::collections::BTreeMap;

use anyhow::Context as _;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::db::generated::{
    mybb_forums::Model as MybbForum, mybb_posts::Model as MybbPost,
    mybb_threads::Model as MybbThread, mybb_users::Model as MybbUser,
};

#[derive(Serialize, Deserialize)]
pub struct DatabaseData {
    pub users: BTreeMap<u32, User>,
    pub forums: BTreeMap<u16, Forum>,
    pub threads: BTreeMap<u32, Thread>,
    pub posts: BTreeMap<u32, Post>,
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

#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub id: u32,
    pub forum_id: u16,
    pub subject: String,
    pub user_id: u32,
    pub creation_date: Timestamp,
}

impl TryFrom<MybbThread> for Thread {
    type Error = anyhow::Error;

    fn try_from(value: MybbThread) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.tid,
            forum_id: value.fid,
            user_id: value.uid,
            subject: value.subject,
            creation_date: Timestamp::from_second(value.dateline).context("convert timestamp")?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub thread_id: u32,
    pub subject: String,
    pub creation_date: Timestamp,
    pub user_id: u32,
    pub message: String,
}

impl TryFrom<MybbPost> for Post {
    type Error = anyhow::Error;

    fn try_from(value: MybbPost) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.pid,
            thread_id: value.tid,
            subject: value.subject,
            creation_date: Timestamp::from_second(value.dateline).context("convert timestamp")?,
            user_id: value.uid,
            message: value.message,
        })
    }
}
