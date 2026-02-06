use std::collections::BTreeMap;

use anyhow::Context as _;
use askama::Template;

use crate::{
    data::{DatabaseData, Post, Thread, User},
    filters,
};

#[derive(Template)]
#[template(path = "thread.html")]
pub struct ThreadTemplate<'a> {
    pub thread: &'a Thread,
    pub posts: Vec<&'a Post>,
    pub users: &'a BTreeMap<u32, User>,
}

impl<'a> TryFrom<(&'a DatabaseData, u32)> for ThreadTemplate<'a> {
    type Error = anyhow::Error;

    fn try_from((data, thread_id): (&'a DatabaseData, u32)) -> Result<Self, Self::Error> {
        Ok(Self {
            thread: data
                .threads
                .values()
                .find(|&t| t.id == thread_id)
                .context("find thread by id")?,

            posts: {
                let mut posts = data
                    .posts
                    .values()
                    .filter(|f| f.thread_id == thread_id)
                    .collect::<Vec<_>>();

                posts.sort_by_key(|t| t.id);

                posts
            },
            users: &data.users,
        })
    }
}
