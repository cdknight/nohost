use maud::{DOCTYPE, html, Markup};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        link rel="stylesheet" href="/static/styles.css";
        title { (title) }
    }
}

pub fn error_page(domain: &str, ip: Option<&str>) -> Markup {
    html! {
        (header(&format!("Domain {} not found", domain)))

        div.inner.error {
            h1 {
                "The domain " (domain) " wasn't found"
            }

            @if let Some(ip) = ip {
                h3 .light {
                    "Your public IP: " (ip)
                }
            }
        }
    }
}

pub fn landing_html(domain: &str) -> Markup {
    html! {
        (header(&format!("Welcome to {}!", domain)))
        div.inner.success { h1 { "Welcome to " (domain)"!" } }
    }
}
