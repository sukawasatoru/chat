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
use juniper::{FieldResult, ID};
use log::{error, info, warn};
use url::Url;

use crate::data::repository::dev_flex_chat_repository::DevFlexChatRepository;
use crate::feature::dev_flex_chat::{
    self, Channel, ChannelInput, ChannelResponse, CommentInput, CommentResponse,
};
use crate::model::juniper_object::Context;
use crate::prelude::*;

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

struct Query {
    #[allow(dead_code)]
    oauth_value: Option<String>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            oauth_value: Default::default(),
        }
    }
}

#[juniper::object(Context = Context)]
impl Query {
    fn channel(&self, context: &Context, id: ID) -> FieldResult<Option<Channel>> {
        dev_flex_chat::channel(&context.chat_repo, id).map_err(|e| {
            warn!("failed to find channel: {:?}", e);
            e
        })
    }

    fn channels(context: &Context) -> FieldResult<Vec<Channel>> {
        dev_flex_chat::channels(&context.chat_repo).map_err(|e| {
            warn!("failed to find channels: {:?}", e);
            e
        })
    }
}

struct Mutation {
    #[allow(dead_code)]
    oauth_value: Option<String>,
}

impl Default for Mutation {
    fn default() -> Self {
        Self {
            oauth_value: Default::default(),
        }
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn add_channel(context: &Context, channel: ChannelInput) -> FieldResult<ChannelResponse> {
        dev_flex_chat::add_channel(&context.chat_repo, channel).map_err(|e| {
            warn!("failed to execute the add_channel: {:?}", e);
            e
        })
    }

    fn add_comment(
        &self,
        context: &Context,
        comment: CommentInput,
    ) -> FieldResult<CommentResponse> {
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
    let database_path = crate::util::get_database_file_path(database);
    let socket_address = address.parse()?;
    info!("database_path: {:?}", database_path);
    info!("socket_address: {:?}", socket_address);
    let chat_repo = DevFlexChatRepository::prepare(database_path)?;

    let context = Arc::new(Context::new(chat_repo));
    let root_node = Arc::new(juniper::RootNode::new(
        Query::default(),
        Mutation::default(),
    ));

    info!(
        "database_version: {:?}",
        context.chat_repo.database_version()?
    );

    hyper::rt::run(
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
    );

    Ok(())
}

fn on_request(
    context: Arc<Context>,
    root_node: Arc<juniper::RootNode<'static, Query, Mutation>>,
    req: Request<Body>,
) -> Fallible<BoxFut> {
    info!("{:?}", req);

    let url = Url::parse(&format!("http://authority{}", req.uri()))?;
    match (req.method(), url.path_segments().ok_or_err()?.next()) {
        (&Method::GET, Some("graphiql")) => Ok(Box::new(
            juniper_hyper::graphiql("/graphql").map(append_access_control_allow_origin_all),
        )),
        (&Method::OPTIONS, Some("graphql")) => {
            warn!("TODO: Support OPTIONS method for juniper");
            Err(failure::format_err!("TODO: Support OPTIONS method"))
        }
        (&Method::GET, Some("graphql")) => Ok(Box::new(
            juniper_hyper::graphql(root_node, context, req)
                .map(append_access_control_allow_origin_all),
        )),
        (&Method::POST, Some("graphql")) => {
            // TODO: support oauth.
            let root_node = match req.headers().get(hyper::header::AUTHORIZATION) {
                Some(data) => {
                    let o_auth_str = data.to_str()?;
                    Arc::new(juniper::RootNode::new(
                        Query {
                            oauth_value: Some(o_auth_str.to_owned()),
                        },
                        Mutation {
                            oauth_value: Some(o_auth_str.into()),
                        },
                    ))
                }
                None => root_node,
            };

            Ok(Box::new(
                juniper_hyper::graphql(root_node, context, req)
                    .map(append_access_control_allow_origin_all),
            ))
        }
        _ => {
            info!("404");
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(Box::new(futures::future::ok(response)))
        }
    }
}

fn append_access_control_allow_origin_all(mut response: Response<Body>) -> Response<Body> {
    response.headers_mut().append(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        "*".parse().unwrap(),
    );
    response
}
