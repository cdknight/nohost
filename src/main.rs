use actix_web::{get, post, web, guard, dev, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::env;
extern crate regex;
#[macro_use] extern crate lazy_static;

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
    let ip = conninfo.remote_addr().unwrap(); // Hopefully this works

    let mut resp = format!("Domain {} not found", &CONFIG.domain);
    if CONFIG.show_ip {
        resp += &format!("\nYour Public IP: {}", ip);
    }

    HttpResponse::NotFound().body(resp)
}

async fn landing(req: HttpRequest) -> impl Responder {
    format!("Welcome to {}!", CONFIG.domain)
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
        App::new()
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

