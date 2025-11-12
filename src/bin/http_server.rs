use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

type Headers = HashMap<String, String>;

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    version: String,
    headers: Headers,
    body: String,
    query_params: HashMap<String, String>,
}

#[derive(Debug)]
struct Response {
    status_code: u16,
    status_text: String,
    headers: Headers,
    body: String,
}

impl Response {
    fn new(status_code: u16, body: String) -> Self {
        let status_text = match status_code {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            _ => "Unknown",
        }.to_string();

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Connection".to_string(), "close".to_string());

        Response {
            status_code,
            status_text,
            headers,
            body,
        }
    }

    fn to_string(&self) -> String {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code, self.status_text
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        response.push_str(&self.body);

        response
    }
}

fn parse_request(buffer: &[u8]) -> Result<Request, String> {
    let text = String::from_utf8_lossy(buffer);
    let lines: Vec<&str> = text.split("\r\n").collect();

    if lines.is_empty() {
        return Err("Empty request".to_string());
    }

    // Parse request line
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if request_line_parts.len() < 3 {
        return Err("Invalid request line".to_string());
    }

    let method = request_line_parts[0].to_string();
    let full_path = request_line_parts[1];
    let version = request_line_parts[2].to_string();

    // Parse path and query parameters
    let (path, query_params) = parse_path(full_path);

    // Parse headers
    let mut headers = HashMap::new();
    let mut body_start = 0;

    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    // Parse body
    let body = lines[body_start..].join("\r\n");

    Ok(Request {
        method,
        path,
        version,
        headers,
        body,
        query_params,
    })
}

fn parse_path(path: &str) -> (String, HashMap<String, String>) {
    if let Some(q_pos) = path.find('?') {
        let path_part = path[..q_pos].to_string();
        let query_part = &path[q_pos + 1..];

        let query_params = query_part
            .split('&')
            .filter_map(|param| {
                let parts: Vec<&str> = param.split('=').collect();
                if parts.len() == 2 {
                    Some((
                        decode_url(&parts[0]).unwrap_or_default(),
                        decode_url(&parts[1]).unwrap_or_default(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        (path_part, query_params)
    } else {
        (path.to_string(), HashMap::new())
    }
}

fn decode_url(s: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let hex: String = chars
                    .by_ref()
                    .take(2)
                    .collect();
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                } else {
                    return Err("Invalid URL encoding".to_string());
                }
            }
            '+' => result.push(' '),
            _ => result.push(c),
        }
    }

    Ok(result)
}

fn handle_request(req: &Request) -> Response {
    let path = req.path.as_str();
    let method = req.method.as_str();

    match (method, path) {
        ("GET", "/get") => handle_get(req),
        ("POST", "/post") => handle_post(req),
        ("PUT", "/put") => handle_put(req),
        ("DELETE", "/delete") => handle_delete(req),
        ("GET", "/headers") => handle_headers(req),
        ("GET", "/user-agent") => handle_user_agent(req),
        ("GET", "/ip") => handle_ip(req),
        ("GET", "/delay") => handle_delay(req),
        ("GET", "/status") => handle_status_code(req),
        ("GET", "/json") => handle_json(req),
        ("POST", "/json") => handle_post_json(req),
        ("GET", "/") => handle_root(),
        _ => Response::new(404, json_error("Not Found")),
    }
}

fn handle_root() -> Response {
    let body = r#"{"message":"Welcome to HTTPBin clone","endpoints":["GET /get","POST /post","PUT /put","DELETE /delete","GET /headers","GET /user-agent","GET /ip","GET /delay?seconds=N","GET /status?code=N","GET /json","POST /json"]}"#;
    Response::new(200, body.to_string())
}

fn handle_get(req: &Request) -> Response {
    let mut json = String::from("{\"args\":{");

    let args: Vec<String> = req
        .query_params
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, escape_json(v)))
        .collect();
    json.push_str(&args.join(","));

    json.push_str("},\"headers\":{");

    let headers: Vec<String> = req
        .headers
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, escape_json(v)))
        .collect();
    json.push_str(&headers.join(","));

    json.push_str("},\"url\":\"http://localhost:3000");
    json.push_str(&req.path);
    json.push('"');
    json.push('}');

    Response::new(200, json)
}

fn handle_post(req: &Request) -> Response {
    let mut json = String::from("{\"args\":{");

    let args: Vec<String> = req
        .query_params
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, escape_json(v)))
        .collect();
    json.push_str(&args.join(","));

    json.push_str("},\"form\":{},\"data\":\"");
    json.push_str(&escape_json(&req.body));
    json.push_str("\",\"files\":{},\"json\":null,\"url\":\"http://localhost:3000");
    json.push_str(&req.path);
    json.push('"');
    json.push('}');

    Response::new(200, json)
}

fn handle_put(req: &Request) -> Response {
    let mut json = String::from("{\"args\":{");

    let args: Vec<String> = req
        .query_params
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, escape_json(v)))
        .collect();
    json.push_str(&args.join(","));

    json.push_str("},\"data\":\"");
    json.push_str(&escape_json(&req.body));
    json.push_str("\",\"url\":\"http://localhost:3000");
    json.push_str(&req.path);
    json.push('"');
    json.push('}');

    Response::new(200, json)
}

fn handle_delete(req: &Request) -> Response {
    let json = format!(
        r#"{{"args":{{}},"data":"{}","url":"http://localhost:3000{}"}}"#,
        escape_json(&req.body),
        &req.path
    );
    Response::new(200, json)
}

fn handle_headers(req: &Request) -> Response {
    let mut json = String::from("{\"headers\":{");

    let headers: Vec<String> = req
        .headers
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, escape_json(v)))
        .collect();
    json.push_str(&headers.join(","));

    json.push('}');
    json.push('}');

    Response::new(200, json)
}

fn handle_user_agent(req: &Request) -> Response {
    let ua = req
        .headers
        .get("User-Agent")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());

    let json = format!(r#"{{"user-agent":"{}"}}"#, escape_json(&ua));
    Response::new(200, json)
}

fn handle_ip(_req: &Request) -> Response {
    let json = r#"{"origin":"127.0.0.1"}"#.to_string();
    Response::new(200, json)
}

fn handle_delay(req: &Request) -> Response {
    let seconds = req
        .query_params
        .get("seconds")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0)
        .min(10);

    thread::sleep(std::time::Duration::from_secs(seconds));

    let json = r#"{"delayed":true}"#.to_string();
    Response::new(200, json)
}

fn handle_status_code(req: &Request) -> Response {
    let code = req
        .query_params
        .get("code")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(200);

    let body = format!(r#"{{"code":{}}}"#, code);
    Response::new(code, body)
}

fn handle_json(_req: &Request) -> Response {
    let json = r#"{"slideshow":{"author":"Yours Truly","date":"date of publication","slides":[{"title":"Wake up to WonderWidgets!","type":"all"}]}}"#.to_string();
    Response::new(200, json)
}

fn handle_post_json(req: &Request) -> Response {
    let json = format!(
        r#"{{"json":{},"data":"{}"}}"#,
        req.body,
        escape_json(&req.body)
    );
    Response::new(200, json)
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn json_error(msg: &str) -> String {
    format!(r#"{{"error":"{}"}}"#, msg)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    if let Ok(n) = stream.read(&mut buffer) {
        if n > 0 {
            match parse_request(&buffer[..n]) {
                Ok(req) => {
                    let response = handle_request(&req);
                    let _ = stream.write_all(response.to_string().as_bytes());
                }
                Err(_) => {
                    let error_resp =
                        Response::new(400, json_error("Bad Request"));
                    let _ = stream.write_all(error_resp.to_string().as_bytes());
                }
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .expect("Failed to bind to port 3000");

    println!("Server listening on http://127.0.0.1:3000");
    println!("Available endpoints:");
    println!("  GET  /get");
    println!("  POST /post");
    println!("  PUT  /put");
    println!("  DELETE /delete");
    println!("  GET  /headers");
    println!("  GET  /user-agent");
    println!("  GET  /ip");
    println!("  GET  /delay?seconds=N");
    println!("  GET  /status?code=N");
    println!("  GET  /json");
    println!("  POST /json");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
