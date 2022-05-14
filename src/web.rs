use rouille::Request;
use rouille::Response;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::thread;
use mime_guess;
use serde_json::{Map, Value};
use crate::cli;
use std::env;
use std::path::{Path, PathBuf};

pub fn start(port: i32, pageMap : Map<String, Value>) {
    let start = std::time::Instant::now();
    let rouille_thread = thread::spawn(move || {
        rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
            let url = request.url();
            let response = match url.as_str() {
                "/" => {
                    let mut file_js_path = PathBuf::new();
                    file_js_path.push(env::current_dir().unwrap());
                    file_js_path.push("src");
                    file_js_path.push(pageMap.get("/").unwrap().as_str().unwrap());
                    let mut file = File::open(file_js_path).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    Response::html(content.replace(".ts", ".js"))
                },
                _ => {
                    // if it's some other page instead of index
                    if let Some(page) = pageMap.get(url.as_str()) {
                        let mut file_js_path = PathBuf::new();
                        file_js_path.push(env::current_dir().unwrap());
                        file_js_path.push("src");
                        file_js_path.push(page.as_str().unwrap());
                        let mut file = File::open(file_js_path).unwrap();
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        Response::html(content.replace(".ts", ".js"))
                    }
                    // if it's neither of those, try to send a file from dist
                    else {
                        let file = File::open(format!("dist/{}", url.as_str().replace("/", "")));
                        match file {
                            Ok(file) => {
                                let content_type = mime_guess::from_path(format!("dist/{}", url.as_str().replace("/", ""))).first_or_octet_stream();
                                return Response::from_file(content_type.to_string(), file);
                            },
                            Err(_) => Response::text("Not found.").with_status_code(404),
                        }
                    }
                }
            };
            response
        });
    });
    let duration = start.elapsed().as_micros();
    println!(
        "🚀 Running on port {}. Took {} microseconds!",
        port, duration
    );
    drop(start);
}
