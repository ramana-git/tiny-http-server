use ascii::AsciiString;
use config::Config;
use std::{env, fs};
use std::ffi::OsStr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use tiny_http::{Header, Response, Server, StatusCode};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let message = format!("{:?} [port/8888] [docroot/.] [content-types]", args[0]);

    let port = if args.len() > 1 {
        args[1].parse().expect(&message)
    } else {
        8888
    };

    let docroot = if args.len() > 2 {
        if !Path::new(&args[2]).is_dir() {
            println!("{message}");
            return;
        }
        &args[2]
    } else {
        "."
    };

    let types = if args.len() > 3 {
        let config = Path::new(&args[3]);
        if !config.is_file() {
            println!("{message}");
            return;
        }
        Config::builder().add_source(config::File::from(config)).build().expect(&message)
    } else {
        let default_settings=Config::builder()
        .set_default("css", "text/css".to_string()).unwrap()
        .set_default("gif", "image/gif".to_string()).unwrap()
        .set_default("htm", "text/html; charset=utf8".to_string()).unwrap()
        .set_default("html", "text/html; charset=utf8".to_string()).unwrap()
        .set_default("jpeg", "image/jpeg".to_string()).unwrap()
        .set_default("jpg", "image/jpeg".to_string()).unwrap()
        .set_default("js", "text/javascript".to_string()).unwrap()
        .set_default("json", "application/json".to_string()).unwrap()
        .set_default("pdf", "application/pdf".to_string()).unwrap()
        .set_default("png", "image/png".to_string()).unwrap()
        .set_default("svg", "image/svg+xml".to_string()).unwrap()
        .set_default("txt", "text/plain; charset=utf8".to_string()).unwrap().build().unwrap();
        default_settings
    };

    let server = Server::http(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).unwrap();
    println!("Server listening on port {port} and serving from {docroot:?}");

    let default_content_type = String::from("application/unknown");

    loop {
        let mut rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break,
        };

        println!("\n{} {} {}", rq.method(), rq.url(), rq.http_version());
        for header in rq.headers() {
            println!("{}: {}", header.field, header.value);
        }
        let mut body = String::new();
        rq.as_reader().read_to_string(&mut body).unwrap();
        println!("\n{body}\n");
        
        let mut str_path=docroot.to_owned();
        str_path.push_str(rq.url());

        let path = Path::new(&str_path);
        let file = fs::File::open(&path);

        if file.is_ok() {
            let content_type = types.get_string(match path.extension(){
                Some(ext) => ext,
                None => OsStr::new("text/plain; charset=utf8")
            }.to_str().unwrap()).unwrap_or(default_content_type.clone());
            let response = Response::from_file(file.unwrap());
            let response = response.with_header(Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_str(&content_type).unwrap(),
            });
            println!("Responding with: {path:?}\n");
            let _ = rq.respond(response);
        } else {
            println!("File not found: {path:?}\n");
            let rep = Response::new_empty(StatusCode(404));
            let _ = rq.respond(rep);
        }
    }
}