use actix_web::{get, post, web, guard, dev, App, HttpResponse, HttpServer, Responder, HttpRequest, Result as AwResult};
use maud::Markup;
use actix_web_static_files::ResourceFiles;
use std::env;
use nohost::*;
extern crate regex;
#[macro_use] extern crate lazy_static;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

struct Config {
    domain: String,
    bind_addr: String,
    show_ip: bool // Show the IP in the failed domain fetch
}

lazy_static! {
    static ref CONFIG: Config = Config {
        domain: env::var("NOHOST_DOMAIN").expect("You must provide a nohost domain."),
        bind_addr: env::var("NOHOST_BINDADDR").unwrap_or("127.0.0.1:8080".to_string()),
        show_ip: match env::var("NOHOST_SHOWIP").unwrap_or("1".to_owned()).as_str() { // By default, show the IP
            "1" => true,
            _ => false
        }
    };
}

async fn domain_notfound(req: HttpRequest) -> impl Responder {
    let host = req.headers().get("Host").unwrap().to_str().unwrap(); // We know this will exist because the guard guarantees it does
    let conninfo = req.connection_info();
    let mut ip = None;

    if CONFIG.show_ip {
        ip = Some(conninfo.realip_remote_addr().unwrap()); // Hopefully this works
    }

    HttpResponse::NotFound().body(error_page(&host, ip).into_string())
}

async fn landing(req: HttpRequest) -> AwResult<Markup> {
    Ok(landing_html(&CONFIG.domain))
}

fn check_wildcard(req: &dev::RequestHead) -> bool {
    if req.headers.contains_key("Host") {
        let host = req.headers.get("Host").unwrap().to_str().unwrap();
        println!("REQ: Incoming host is {}", host);
        // Match wildcard header

        let re = regex::Regex::new(&format!(r"[a-zA-Z\d+]\.{}", CONFIG.domain)).unwrap();
        return re.is_match(host)
    }
    false
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(move || {
        let generated = generate();

        App::new()
            .service(
                ResourceFiles::new("/static", generated)
            )
            .service(
                web::scope("/")
                    .guard(guard::fn_guard(check_wildcard))
                    .route("", web::get().to(domain_notfound)) // Not found
                )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", &CONFIG.domain))
                    .route("", web::get().to(landing))
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind(&CONFIG.bind_addr)?
    .run()
    .await
}

