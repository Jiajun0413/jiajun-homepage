//! Site entry: Axum + Dioxus SSR under the `server` feature; otherwise `launch` in the browser.

mod app;
mod model;
mod site;

// The build-time content validator is shared by build.rs (#[path]) and the test target.
#[cfg(test)]
#[path = "content.rs"]
mod content;

pub use app::{App, Route};

#[cfg(feature = "server")]
mod server {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use axum::Router;
    use dioxus::prelude::*;
    use dioxus_server::{DioxusRouterExt, ServeConfig};

    use crate::{App, feed_xml};

    pub fn main() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        runtime.block_on(async {
            let app = Router::new()
                .route(
                    "/feed.xml",
                    axum::routing::get(|| async {
                        (
                            [("content-type", "application/rss+xml; charset=utf-8")],
                            feed_xml(),
                        )
                    }),
                )
                .fallback_service(Router::new().serve_dioxus_application(ServeConfig::new(), App));

            let addr = address();
            let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .unwrap_or_else(|err| panic!("bind {addr}: {err}"));
            let shutdown = async {
                #[cfg(unix)]
                {
                    use tokio::signal::unix::{SignalKind, signal};
                    let mut term = signal(SignalKind::terminate()).expect("SIGTERM handler");
                    tokio::select! {
                        _ = tokio::signal::ctrl_c() => {}
                        _ = term.recv() => {}
                    }
                }
                #[cfg(not(unix))]
                {
                    let _ = tokio::signal::ctrl_c().await;
                }
            };
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
            )
            .with_graceful_shutdown(shutdown)
            .await
            .expect("server error");
        });
    }

    /// Bind loopback by default. `IP`/`PORT` come from dx serve or systemd,
    /// read through dioxus cli_config (same source as fullstack_address_or_localhost).
    /// Invalid explicit values panic instead of falling back silently.
    fn address() -> SocketAddr {
        let ip = match std::env::var("IP") {
            Ok(v) => v
                .parse::<IpAddr>()
                .unwrap_or_else(|err| panic!("invalid IP={v:?}: {err}")),
            Err(_) => dioxus::cli_config::server_ip().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        };
        let port = match std::env::var("PORT") {
            Ok(v) => v
                .parse::<u16>()
                .unwrap_or_else(|err| panic!("invalid PORT={v:?}: {err}")),
            Err(_) => dioxus::cli_config::server_port().unwrap_or(8082),
        };
        SocketAddr::new(ip, port)
    }
}

#[cfg(any(feature = "server", test))]
/// RSS 2.0: titles, links, and summaries reuse the build-time `POSTS`.
/// `pubDate` uses the precomputed RFC822 `rss_date`, not the ISO date.
fn feed_xml() -> String {
    use crate::model::POSTS;
    use crate::site::SITE;

    const BASE: &str = "https://jiajun0805.cc.cd";
    let mut items = String::new();
    for post in POSTS {
        items.push_str(&format!(
            "<item><title>{title}</title><link>{BASE}/blog/{slug}</link>\
             <guid>{BASE}/blog/{slug}</guid><pubDate>{date}</pubDate>\
             <description>{summary}</description></item>",
            title = escape_xml(post.title),
            slug = escape_xml(post.slug),
            date = escape_xml(post.rss_date),
            summary = escape_xml(post.summary),
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel>\
         <title>{name} · Blog</title><link>{BASE}/</link>\
         <description>{name}'s notes</description><language>en</language>\
         {items}</channel></rss>",
        name = escape_xml(SITE.name),
    )
}

#[cfg(any(feature = "server", test))]
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn main() {
    #[cfg(feature = "server")]
    server::main();

    #[cfg(not(feature = "server"))]
    dioxus::launch(crate::App);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_escape_covers_all_five_entities() {
        assert_eq!(
            escape_xml("a&b<c>d\"e'f"),
            "a&amp;b&lt;c&gt;d&quot;e&apos;f"
        );
        assert_eq!(
            escape_xml("\u{4e2d}\u{6587} \u{6807}\u{9898}"),
            "\u{4e2d}\u{6587} \u{6807}\u{9898}"
        );
    }

    #[test]
    fn feed_is_valid_rss_with_rfc822_pubdates() {
        let feed = feed_xml();
        assert!(feed.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(feed.contains("<rss version=\"2.0\"><channel>"));
        assert!(feed.contains("<language>en</language>"));
        assert!(feed.ends_with("</channel></rss>"));
        assert!(!feed.contains("<script"));

        // Every item's pubDate must be RFC822, not ISO YYYY-MM-DD.
        let mut count = 0;
        for part in feed.split("<pubDate>").skip(1) {
            let date = part.split("</pubDate>").next().unwrap();
            assert!(date.ends_with("00:00:00 GMT"), "non-RFC822 pubDate: {date}");
            let weekday = &date[..3];
            assert!(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].contains(&weekday));
            assert!(!date.contains('-'), "ISO date leaked into feed: {date}");
            count += 1;
        }
        assert_eq!(count, crate::model::POSTS.len());
    }
}
