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

use std::convert::TryInto;

use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject, GraphQLObject, ID};
use log::warn;

use crate::data::db::entity::dev_flex_chat_entity::{
    ChannelEntity, ChannelID, CommentEntity, CommentID,
};
use crate::data::repository::dev_flex_chat_repository::DevFlexChatRepository;
use crate::model::juniper_object::{Context, OrderDirection};

pub struct Channel {
    pub id: ChannelID,
    pub name: String,
}

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct ChannelInput {
    pub name: String,
}

#[derive(GraphQLObject)]
pub struct ChannelResponse {
    pub id: ID,
    pub name: String,
}

#[derive(Debug, juniper::GraphQLInputObject)]
struct CommentOrder {
    direction: OrderDirection,
}

#[derive(GraphQLObject)]
pub struct Comment {
    pub id: ID,
    pub name: String,
    pub message: String,
}

#[derive(GraphQLInputObject)]
pub struct CommentInput {
    pub channel_id: ID,
    pub name: String,
    pub message: String,
}

#[derive(GraphQLObject)]
pub struct CommentResponse {
    pub id: ID,
    pub name: String,
    pub message: String,
}

#[juniper::object(Context = Context)]
impl Channel {
    fn id(&self) -> ID {
        self.id.0.to_string().into()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn comments(
        &self,
        context: &Context,
        first: i32,
        order_by: CommentOrder,
    ) -> FieldResult<Vec<Comment>> {
        let first = first.try_into().map_err(|e| {
            FieldError::new(
                e,
                graphql_value!({"internal_error": "failed to convert number"}),
            )
        })?;
        Ok(context
            .chat_repo
            .retrieve_first(&self.id, first, &order_by.direction)
            .map_err(|e| {
                warn!("failed repo.retrieve_first: {:?}", e);
                FieldError::new(
                    e,
                    graphql_value!({"internal_error": "failed to retrieve data"}),
                )
            })?
            .into_iter()
            .map(|entity| Comment {
                id: entity.id.0.to_string().into(),
                name: entity.name,
                message: entity.message,
            })
            .collect::<Vec<_>>())
    }

    fn comments_long_polling(
        &self,
        context: &Context,
        id: Option<ID>,
        order_by: CommentOrder,
    ) -> FieldResult<Vec<Comment>> {
        match id {
            Some(id) => {
                let id = id.parse().map_err(|e| {
                    FieldError::new(
                        e,
                        graphql_value!({"internal_error": "failed to convert to UUID"}),
                    )
                })?;
                Ok(context
                    .chat_repo
                    .retrieve_after_long_polling(&self.id, &CommentID(id), &order_by.direction)
                    .map_err(|e| {
                        FieldError::new(
                            e,
                            graphql_value!({"internal_error": "failed to long polling"}),
                        )
                    })?
                    .into_iter()
                    .map(|entity| Comment {
                        id: entity.id.0.to_string().into(),
                        name: entity.name,
                        message: entity.message,
                    })
                    .collect::<Vec<_>>())
                .map_err(|e| {
                    warn!("failed to long polling comment: {:?}", e);
                    e
                })
            }
            None => Ok(vec![context
                .chat_repo
                .long_polling(&self.id)
                .map_err(|e| {
                    FieldError::new(
                        e,
                        graphql_value!({"internal_error": "failed to long polling"}),
                    )
                })
                .map(|entity| Comment {
                    id: entity.id.0.to_string().into(),
                    name: entity.name,
                    message: entity.message,
                })?]),
        }
    }
}

pub fn channel(repo: &DevFlexChatRepository, id: ID) -> FieldResult<Option<Channel>> {
    let id = id.parse().map(ChannelID).map_err(|e| {
        FieldError::new(
            e,
            graphql_value!({"internal_error": "failed to convert to UUID"}),
        )
    })?;
    Ok(repo
        .find_channel(&id)
        .map_err(|e| {
            warn!("failed to execute find_channel: {:?}", e);
            FieldError::new(
                e,
                graphql_value!({"internal_error": "failed to find channel"}),
            )
        })?
        .map(|data| Channel {
            id,
            name: data.name,
        }))
}

pub fn channels(repo: &DevFlexChatRepository) -> FieldResult<Vec<Channel>> {
    Ok(repo
        .channels()
        .map_err(|e| {
            warn!("failed to execute channels: {:?}", e);
            FieldError::new(
                e,
                graphql_value!({"internal_error": " failed to find channels"}),
            )
        })?
        .into_iter()
        .map(|entity| Channel {
            id: entity.id,
            name: entity.name,
        })
        .collect::<Vec<_>>())
}

pub fn add_channel(
    repo: &DevFlexChatRepository,
    channel: ChannelInput,
) -> FieldResult<ChannelResponse> {
    let id = uuid::Uuid::new_v4();
    // TODO: check conflict of a uuid.
    // TODO: check conflict of a name.
    repo.save_channel(ChannelEntity {
        id: ChannelID(id),
        name: channel.name.to_owned(),
    })?;
    Ok(ChannelResponse {
        id: id.to_string().into(),
        name: channel.name,
    })
}

pub fn add_comment(
    repo: &DevFlexChatRepository,
    comment: CommentInput,
) -> FieldResult<CommentResponse> {
    let id = uuid::Uuid::new_v4();
    // TODO: check conflict of a uuid.
    repo.save_comment(CommentEntity {
        id: CommentID(id),
        channel_id: ChannelID(comment.channel_id.parse()?),
        name: comment.name.to_owned(),
        message: comment.message.to_owned(),
    })?;
    Ok(CommentResponse {
        id: id.to_string().into(),
        name: comment.name,
        message: comment.message,
    })
}
