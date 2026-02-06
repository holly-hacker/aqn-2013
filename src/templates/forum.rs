use anyhow::Context as _;
use askama::Template;

use crate::data::{DatabaseData, Forum, Thread};

#[derive(Template)]
#[template(path = "forum.html")]
pub struct ForumTemplate<'a> {
    pub forum: &'a Forum,
    pub child_forums: Vec<&'a Forum>,
    pub sticky_threads: Vec<&'a Thread>,
    pub threads: Vec<&'a Thread>,
}

impl<'a> TryFrom<(&'a DatabaseData, u16)> for ForumTemplate<'a> {
    type Error = anyhow::Error;

    fn try_from((data, forum_id): (&'a DatabaseData, u16)) -> Result<Self, Self::Error> {
        Ok(Self {
            forum: data
                .forums
                .values()
                .find(|&f| f.id == forum_id)
                .context("find forum by id")?,
            child_forums: data
                .forums
                .iter()
                .filter(|kvp| kvp.1.parent == Some(forum_id))
                .map(|kvp| kvp.1)
                .collect(),
            sticky_threads: {
                let mut threads = data
                    .threads
                    .values()
                    .filter(|t| t.sticky)
                    .filter(|t| t.forum_id == forum_id)
                    .collect::<Vec<_>>();

                threads.sort_by_key(|t| std::cmp::Reverse(t.id));

                threads
            },
            threads: {
                let mut threads = data
                    .threads
                    .values()
                    .filter(|t| !t.sticky)
                    .filter(|t| t.forum_id == forum_id)
                    .collect::<Vec<_>>();

                threads.sort_by_key(|t| std::cmp::Reverse(t.id));

                threads
            },
        })
    }
}
