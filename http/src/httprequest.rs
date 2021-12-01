use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    version: Version,
    resource: Resource,
    headers: HashMap<String,String>,
    msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {

            } else {
                parsed_msg_body = line
            }
        }

        HttpRequest {
            version: parsed_version,
            resource: parsed_resource,
            method: parsed_method,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}


fn process_req_line (line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_items = line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) =header_items.next() {
        value = v.to_string();
    }
    (
        key,
        value
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_method_into() {
        let get: Method = "GET".into();
        assert_eq!(get, Method::Get);

        let post: Method = "POST".into();
        assert_eq!(post, Method::Post);

        let uninitialized: Method = "wired str".into();
        assert_eq!(uninitialized, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let v1_1: Version = "HTTP/1.1".into();
        let uninitialized: Version = "HTTP/2.0".into();
        assert_eq!(v1_1, Version::V1_1);
        assert_eq!(uninitialized, Version::Uninitialized);
    }

    #[test]
    fn test_http_request_into() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost\r\nAccept: */*\r\nUser-Agent: curl/5.51.1");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/5.51.1".into());
        let req: HttpRequest = s.into();
        
        assert_eq!(Method::Get,req.method);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(headers_expected, req.headers);

    }
}