
use ascii::AsciiString;
//use config::Config;
use std::str::FromStr;
use std::{env, fs};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
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
        let types=HashMap::new();
//        let mut _settings = Config::builder().add_source(config::File::from(config)).build().expect(&message);
//        let _test=_settings.get::<String>("dfg");
        types
    } else {
        let mut types = HashMap::new();
        types.insert("css", "text/css");
        types.insert("gif", "image/gif");
        types.insert("htm", "text/html; charset=utf8");
        types.insert("html", "text/html; charset=utf8");
        types.insert("jpeg", "image/jpeg");
        types.insert("jpg", "image/jpeg");
        types.insert("js", "text/javascript");
        types.insert("json", "application/json");
        types.insert("pdf", "application/pdf");
        types.insert("png", "image/png");
        types.insert("svg", "image/svg+xml");
        types.insert("txt", "text/plain; charset=utf8");
        types
    };

    let server = Server::http(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).unwrap();
    println!("Server listening on port {port} and serving from {docroot:?}");

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
            let response = Response::from_file(file.unwrap());
            let response = response.with_header(Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_str(match path.extension() {
                    None => "text/plain",
                    Some(ext) => match types.get(ext.to_str().unwrap()){
                        Some(ty) => ty,
                        None => "application/unknown",                        
                    },
                }).unwrap(),
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
