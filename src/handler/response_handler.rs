use crate::hyper::{StatusCode, Response, Body};

pub fn response(status_code: StatusCode, body: Body) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(body)
        .unwrap()
}


pub fn response_with_code(status_code: StatusCode) -> Response<Body> {
    response(status_code, Body::empty())
}