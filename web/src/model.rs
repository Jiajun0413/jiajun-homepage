//! 站点事实与内容模型。博客文章在构建期由 `build.rs` 生成（见 `content/posts/`）。

#[derive(Clone, Copy, PartialEq)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub date_display: &'static str,
    pub year: &'static str,
    pub summary: &'static str,
    pub tags: &'static [&'static str],
    pub minutes: usize,
    pub html: &'static str,
    pub rss_date: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/content.rs"));

impl Post {
    pub fn by_slug(slug: &str) -> Option<&'static Self> {
        POSTS.iter().find(|post| post.slug == slug)
    }

    /// 上一篇（更新）/ 下一篇（更旧），列表已按日期倒序。
    pub fn neighbours(&self) -> (Option<&'static Self>, Option<&'static Self>) {
        let Some(index) = POSTS.iter().position(|post| post.slug == self.slug) else {
            return (None, None);
        };
        let newer = index.checked_sub(1).map(|i| &POSTS[i]);
        let older = POSTS.get(index + 1);
        (newer, older)
    }
}

pub struct Project {
    pub name: &'static str,
    pub note: &'static str,
    pub summary: &'static str,
    pub href: &'static str,
    pub link_label: Option<&'static str>,
}

pub struct Site {
    pub name: &'static str,
    pub name_latin: &'static str,
    pub meta_line: &'static str,
    pub github: &'static str,
    pub email: &'static str,
}
