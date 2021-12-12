use std::io::prelude::*;
use http::httprequest::{HttpRequest, Method, Resource};
use http::httpresponse::{HttpResponse};
use super::handler::{ Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler };

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let response: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = response.send_response(stream);
                        },
                        _ => {
                            let response = StaticPageHandler::handle(&req);
                            let _ = response.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let response: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = response.send_response(stream);
            }
        }
    }
}