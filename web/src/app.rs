//! 页面骨架与路由。所有条目数据来自 `model::POSTS` 与 `site::PROJECTS`。

use dioxus::prelude::*;

use crate::model::{Post, POSTS};
use crate::site::{PROJECTS, SITE};

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const RECENT_ON_POSTS: usize = 5;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Archive {},
    #[route("/blog/:slug")]
    PostPage { slug: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "{SITE.name} · {SITE.name_latin}" }
        document::Meta {
            name: "description",
            content: "{SITE.name} 的个人主页：文章、项目与论文翻译工具。"
        }
        document::Meta { name: "color-scheme", content: "light dark" }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// 全站栅格：编号栏 + 正文列。页面内容全部落在 `main` 内。
#[component]
fn Shell(children: Element) -> Element {
    rsx! {
        div { class: "page",
            header { class: "masthead",
                Link { class: "masthead-name", to: Route::Home {},
                    span { class: "masthead-cn", "{SITE.name}" }
                    span { class: "masthead-latin", "{SITE.name_latin}" }
                }
                nav { class: "nav", "aria-label": "主导航",
                    Link { to: Route::Archive {}, "文章" }
                    a { href: "/translate/", "翻译" }
                    a { href: "/#projects", "项目" }
                    a { href: SITE.github, target: "_blank", rel: "noopener", "GitHub ↗" }
                }
            }
            main { class: "column", {children} }
            footer { class: "footer",
                p { class: "footer-meta", "{SITE.meta_line}" }
                div { class: "footer-links",
                    a { href: "mailto:{SITE.email}", "{SITE.email}" }
                    a { href: SITE.github, target: "_blank", rel: "noopener", "GitHub ↗" }
                    a { href: "https://github.com/Jiajun0413/jiajun-homepage", target: "_blank", rel: "noopener", "源码 ↗" }
                    a { href: "/feed.xml", "RSS ↗" }
                }
            }
        }
    }
}

/// 编号悬挂在左栏，标题与计数落在右栏页眉。
#[component]
fn Section(num: &'static str, label: &'static str, count: String, children: Element) -> Element {
    rsx! {
        section { class: "section",
            div { class: "section-head",
                span { class: "section-num", "{num}" }
                h2 { class: "section-label", "{label}" }
                span { class: "section-count", "{count}" }
            }
            div { class: "section-body", {children} }
        }
    }
}

/// 细线索引行：标题 — 引线 — 等宽日期。
#[component]
fn PostRow(post: &'static Post) -> Element {
    rsx! {
        li {
            Link { class: "index-row", to: Route::PostPage { slug: post.slug.to_string() },
                span { class: "index-title", "{post.title}" }
                span { class: "index-leader", "aria-hidden": "true" }
                span { class: "index-date", "{post.date_display}" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    // 首条已被头条占用，索引补充其后的若干条
    let recent: Vec<&Post> = POSTS.iter().skip(1).take(RECENT_ON_POSTS).collect();
    rsx! {
        Shell {
            h1 { class: "visually-hidden", "{SITE.name}" }
            Section { num: "01", label: "文章", count: format!("{}篇", POSTS.len()),
                if let Some(latest) = POSTS.first() {
                    article { class: "lead",
                        Link { class: "lead-title", to: Route::PostPage { slug: latest.slug.to_string() },
                            "{latest.title}"
                        }
                        p { class: "lead-summary", "{latest.summary}" }
                        p { class: "lead-meta",
                            span { "{latest.date_display}" }
                            span { class: "lead-sep", "aria-hidden": "true" }
                            span { "{latest.minutes} 分钟" }
                            if !latest.tags.is_empty() {
                                span { class: "lead-sep", "aria-hidden": "true" }
                                span { {latest.tags.join("、")} }
                            }
                        }
                    }
                }
                if !recent.is_empty() {
                    ul { class: "index",
                        for post in recent.iter() {
                            PostRow { post: *post }
                        }
                    }
                }
                if POSTS.is_empty() {
                    p { class: "empty", "暂无文章。" }
                }
                Link { class: "more", to: Route::Archive {}, "全部文章 →" }
            }
            Section { num: "02", label: "项目", count: format!("{:02}", PROJECTS.len()),
                ul { class: "projects", id: "projects",
                    for project in PROJECTS.iter() {
                        li { class: "project",
                            p { class: "project-line",
                                a { class: "project-name", href: project.href,
                                    target: if project.href.starts_with('/') { "_self" } else { "_blank" },
                                    rel: "noopener",
                                    "{project.name}"
                                }
                                span { class: "project-leader", "aria-hidden": "true" }
                                span { class: "project-note", "{project.note}" }
                            }
                            p { class: "project-summary", "{project.summary}" }
                            if let Some(label) = project.link_label {
                                a { class: "project-link", href: project.href,
                                    target: if project.href.starts_with('/') { "_self" } else { "_blank" },
                                    rel: "noopener",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
                if PROJECTS.is_empty() {
                    p { class: "empty", "暂无项目。" }
                }
            }
        }
    }
}

#[component]
fn Archive() -> Element {
    let mut years: Vec<(&'static str, Vec<&Post>)> = Vec::new();
    for post in POSTS {
        match years.iter_mut().find(|(year, _)| *year == post.year) {
            Some((_, group)) => group.push(post),
            None => years.push((post.year, vec![post])),
        }
    }
    rsx! {
        document::Title { "文章 · {SITE.name}" }
        Shell {
            header { class: "page-head",
                h1 { class: "page-title", "文章" }
                p { class: "page-head-meta", "{POSTS.len()}篇 · 按年" }
            }
            for (year, group) in years.iter() {
                div { class: "year",
                    h2 { class: "year-label", "{year}" }
                    ul { class: "index",
                        for post in group.iter() {
                            PostRow { post: *post }
                        }
                    }
                }
            }
            if POSTS.is_empty() {
                p { class: "empty", "暂无文章。" }
            }
        }
    }
}

#[component]
fn PostPage(slug: String) -> Element {
    let Some(post) = Post::by_slug(&slug) else {
        return rsx! { NotFound { segments: vec![slug] } };
    };
    let (newer, older) = post.neighbours();
    rsx! {
        document::Title { "{post.title} · {SITE.name}" }
        Shell {
            article { class: "post",
                p { class: "post-folio", "aria-hidden": "true",
                    span { class: "post-folio-year", "{post.year}" }
                    span { class: "post-folio-date", "{post.date_display}" }
                }
                header { class: "post-head",
                    h1 { class: "post-title", "{post.title}" }
                    p { class: "post-meta",
                        span { class: "post-meta-date", "{post.date_display}" }
                        span { "{post.minutes} 分钟" }
                        if !post.tags.is_empty() {
                            span { {post.tags.join("、")} }
                        }
                    }
                }
                div { class: "prose", dangerous_inner_html: post.html }
            }
            nav { class: "post-nav", "aria-label": "文章导航",
                if let Some(newer) = newer {
                    Link { class: "post-nav-row", to: Route::PostPage { slug: newer.slug.to_string() },
                        span { class: "post-nav-lab", "较新" }
                        span { class: "post-nav-title", "{newer.title}" }
                    }
                }
                if let Some(older) = older {
                    Link { class: "post-nav-row post-nav-older", to: Route::PostPage { slug: older.slug.to_string() },
                        span { class: "post-nav-lab", "较旧" }
                        span { class: "post-nav-title", "{older.title}" }
                    }
                }
            }
            div { class: "post-foot",
                Link { class: "more", to: Route::Archive {}, "← 全部文章" }
            }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    rsx! {
        document::Title { "页面不存在 · {SITE.name}" }
        Shell {
            div { class: "notice",
                p { class: "notice-code", "404" }
                h1 { class: "notice-title", "页面不存在" }
                p { class: "empty", "{path} 没有对应内容。" }
                Link { class: "more", to: Route::Home {}, "返回首页" }
            }
        }
    }
}
