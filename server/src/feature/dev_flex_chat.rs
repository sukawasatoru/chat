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

use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject, GraphQLObject};

use crate::data::db::entity::dev_flex_chat_entity::CommentEntity;
use crate::data::repository::dev_flex_chat_repository::DevFlexChatRepository;
use log::warn;

#[derive(GraphQLObject)]
pub struct Comment {
    pub id: String,
    pub name: String,
    pub message: String,
}

#[derive(GraphQLInputObject)]
pub struct CommentInput {
    pub name: String,
    pub message: String,
}

#[derive(GraphQLObject)]
pub struct CommentResponse {
    pub id: String,
    pub name: String,
    pub message: String,
}

pub fn comments(repo: &DevFlexChatRepository, first: i32) -> FieldResult<Vec<Comment>> {
    let first = first.try_into().map_err(|e| {
        FieldError::new(
            e,
            graphql_value!({"internal_error": "failed to convert number"}),
        )
    })?;
    Ok(repo
        .retrieve_first(first)
        .map_err(|e| {
            warn!("failed repo.retrieve_first: {:?}", e);
            FieldError::new(
                e,
                graphql_value!({"internal_error": "failed to retrieve data"}),
            )
        })?
        .into_iter()
        .map(|entity| Comment {
            id: entity.id.to_string(),
            name: entity.name,
            message: entity.message,
        })
        .collect::<Vec<_>>())
}

pub fn add_comment(
    repo: &DevFlexChatRepository,
    comment: CommentInput,
) -> FieldResult<CommentResponse> {
    let id = uuid::Uuid::new_v4();
    repo.save_comment(CommentEntity {
        id: id.clone(),
        name: comment.name.to_owned(),
        message: comment.message.to_owned(),
    })?;
    Ok(CommentResponse {
        id: id.to_string(),
        name: comment.name,
        message: comment.message,
    })
}
