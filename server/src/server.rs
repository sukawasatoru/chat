/*
 * Copyright 2020 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::PathBuf;
use std::sync::Arc;

use futures::Future;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};
use juniper::FieldResult;
use log::{error, info, warn};
use url::Url;

use crate::data::repository::dev_flex_chat_repository::DevFlexChatRepository;
use crate::feature::dev_flex_chat::{self, Comment, CommentInput, CommentResponse};
use crate::prelude::*;

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Context {
    chat_repo: DevFlexChatRepository,
}

impl juniper::Context for Context {
    // do nothing.
}

impl Context {
    fn new(chat_repo: DevFlexChatRepository) -> Self {
        Self { chat_repo }
    }
}

struct Query {
    #[allow(dead_code)]
    hello: String,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            hello: "Hello".into(),
        }
    }
}

#[juniper::object(Context = Context)]
impl Query {
    fn api_version() -> &str {
        "0.1"
    }

    fn comments(context: &Context, first: i32) -> FieldResult<Vec<Comment>> {
        match dev_flex_chat::comments(&context.chat_repo, first) {
            Ok(data) => Ok(data),
            Err(e) => {
                warn!("failed to retrieve comment: {:?}", e);
                Err(e)
            }
        }
    }
}

struct Mutation {
    #[allow(dead_code)]
    hello: String,
}

impl Default for Mutation {
    fn default() -> Self {
        Self {
            hello: "Hello".into(),
        }
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn api_version() -> &str {
        "0.1"
    }

    fn add_comment(context: &Context, comment: CommentInput) -> FieldResult<CommentResponse> {
        match dev_flex_chat::add_comment(&context.chat_repo, comment) {
            Ok(data) => Ok(data),
            Err(e) => {
                warn!("failed to execute the add_comment: {:?}", e);
                Err(e)
            }
        }
    }
}

pub fn server(database: Option<PathBuf>, address: String, hostname: String) -> Fallible<()> {
    let database_path = database.unwrap_or(std::path::Path::new("database.toml").to_owned());
    let socket_address = address.parse()?;
    info!("database_path: {:?}", database_path);
    info!("socket_address: {:?}", socket_address);
    let chat_repo = DevFlexChatRepository::new(database_path);

    let context = Arc::new(Context::new(chat_repo));
    let root_node = Arc::new(juniper::RootNode::new(
        Query::default(),
        Mutation::default(),
    ));
    Ok(hyper::rt::run(
        hyper::Server::bind(&socket_address)
            .serve(make_service_fn(move |_| {
                let context = context.clone();
                let root_node = root_node.clone();
                service_fn(
                    move |req| match on_request(context.clone(), root_node.clone(), req) {
                        Ok(data) => data,
                        Err(e) => {
                            warn!("5xx: {:?}", e);
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = hyper::Body::from(format!("{:?}", e));
                            Box::new(futures::future::ok(response))
                        }
                    },
                )
            }))
            .map_err(|e| error!("fatal error: {:?}", e)),
    ))
}

fn on_request(
    context: Arc<Context>,
    root_node: Arc<juniper::RootNode<'static, Query, Mutation>>,
    req: Request<Body>,
) -> Fallible<BoxFut> {
    info!("{:?}", req);

    let url = Url::parse(&format!("http://authority{}", req.uri()))?;
    match (req.method(), url.path_segments().ok_or_err()?.next()) {
        (&Method::GET, Some("graphiql")) => Ok(Box::new(juniper_hyper::graphiql("/graphql").map(
            |mut data| {
                data.headers_mut().append(
                    hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    hyper::header::HeaderValue::from_str("*").unwrap(),
                );
                data
            },
        ))),
        (&Method::OPTIONS, Some("graphql")) => {
            warn!("TODO: Support OPTIONS method for juniper");
            Err(failure::format_err!("TODO: Support OPTIONS method"))
        }
        (&Method::GET, Some("graphql")) => Ok(Box::new(
            juniper_hyper::graphql(root_node, context, req).map(|mut data| {
                data.headers_mut().append(
                    hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    hyper::header::HeaderValue::from_str("*").unwrap(),
                );
                data
            }),
        )),
        (&Method::POST, Some("graphql")) => Ok(Box::new(
            juniper_hyper::graphql(root_node, context, req).map(|mut data| {
                data.headers_mut().append(
                    hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    hyper::header::HeaderValue::from_str("*").unwrap(),
                );
                data
            }),
        )),
        _ => {
            info!("404");
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(Box::new(futures::future::ok(response)))
        }
    }
}
