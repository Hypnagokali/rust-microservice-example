use crate::data::user::UserId;
use crate::{pages::html_pages::INDEX_PAGE, data::{user::{UserData}, repository::{BasicRepository}}};
use crate::handler::response_handler::{response, response_with_code};
use crate::config::global_config::USER_PATH;

use futures::{Future, future};
use std::{result::Result};
use hyper::{Request, Body, Method, Response, StatusCode, http::Error};

fn parse_resource_id(resource_base_path: &str, path: &str) -> Option<usize> {
    path.trim_start_matches(resource_base_path)
        .parse::<UserId>()
        .ok()
        .map(|id| id as usize)
}


pub fn default_handler(req: Request<Body>, data_source: &dyn BasicRepository<UserData, UserId>) -> impl Future<Output = Result<Response<Body>, Error>> {
    // todo: inject controller trait
    // inject data_source in controller
    // and: let res = controller.getResponse(req)

    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            response(StatusCode::OK, Body::from(INDEX_PAGE))
        },
        (method, path) if path.starts_with(USER_PATH)=> {
            let user_id = parse_resource_id(
                format!("{}/", USER_PATH).as_str(), 
                &path
            );

            match (method, user_id) {
                (&Method::DELETE, Some(id)) => {
                    let result = data_source.delete_by_id(id as UserId);

                    match result {
                        Ok(_) => {
                            response_with_code(StatusCode::NO_CONTENT)
                        }
                        Err(e) => {
                            response(StatusCode::BAD_REQUEST, Body::from(format!("{{ \"msg\":\"{}\" }}\n", e)))
                        }
                    }
                },
                (&Method::PUT, Some(id)) => {
                    let user_data = data_source.find_by_id(id as UserId);
                    match user_data {
                        Ok(data) => {
                            let putted_user_data = UserData {
                                id: data.id,
                            };
                            
                            let saved_data = data_source.save(&putted_user_data);

                            if let Ok(res_data) = saved_data {
                                Response::new(Body::from(res_data.to_string()))
                            } else {
                                response_with_code(StatusCode::INTERNAL_SERVER_ERROR)
                            }

                        }
                        Err(e) => {
                            response(StatusCode::NOT_FOUND, Body::from(format!("{{ \"msg\":\"{}\" }}\n", e)))
                        }
                    }
                },
                (&Method::GET, Some(id)) => {
                    match data_source.find_by_id(id as u64) {
                        Ok(data) => {
                            Response::new(Body::from(data.to_string()))
                        },
                        Err(e) => {
                            let msg = format!("{{ \"msg\":\"{}\" }}\n", e);
                            response(StatusCode::NOT_FOUND, Body::from(msg))
                        }
                    }
                },
                (&Method::POST, Some(_)) => {
                    response_with_code(StatusCode::BAD_REQUEST)
                },
                (&Method::POST, None) => {
                    let user_data = UserData {
                        id: 0,
                    };
                    let saved_user = data_source.save(&user_data);
                    Response::new(Body::from(format!("{{ \"user_id\":{} }}\n", saved_user.unwrap().id.to_string())))
                },
                _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED)
            }

            // hier einen lock und den User rausholen
            // let users_mutex = users_ref.lock();
            // let user = users_mutex.unwrap().get(user_id.unwrap());
            // todo!()
        },
        _ => {
            response(StatusCode::NOT_FOUND, Body::from("<h1 style=\"color:red;\">Seite nicht gefunden</h1>"))
        }
    };

    future::ok(res)
    
}