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

use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject, GraphQLObject, ID};
use serde_derive::{Deserialize, Serialize};

use crate::data::repository::user_repository::UserRepository;
use crate::model::juniper_object::Context;
use crate::model::{self, Email};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserID(String);

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmailAddress(String);

pub struct User {
    pub id: UserID,
}

#[derive(GraphQLObject)]
pub struct EmailResponse {
    pub email: String,
    pub visibility: bool,
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    pub email: String,
}

#[derive(GraphQLObject)]
pub struct UserResponse {
    pub id: ID,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> ID {
        self.id.0.to_owned().into()
    }

    fn emails(&self, context: &Context) -> FieldResult<Vec<EmailResponse>> {
        let user = context.user_repo.find_user(&self.id.0).map_err(|e| {
            FieldError::new(
                e,
                graphql_value!({"internal_error": "failed to retrieve user"}),
            )
        })?;

        let user = match user {
            Some(user) => user,
            None => return Ok(vec![]),
        };

        Ok(user
            .email_addresses
            .into_iter()
            .map(|data| EmailResponse {
                email: data.0,
                visibility: false,
            })
            .collect())
    }
}

pub fn add_user(repo: &UserRepository, user: UserInput) -> FieldResult<UserResponse> {
    if !repo
        .find_user_by_emails(&[Email(user.email.to_owned())])?
        .is_empty()
    {
        return Err(FieldError::new(
            failure::format_err!("email already associated another account"),
            graphql_value!({"internal_error": "email already associated another account"}),
        ));
    }

    let id = model::UserID(uuid::Uuid::new_v4().to_string());
    repo.save_user(model::User {
        id: id.clone(),
        display_name: user.email.to_owned(),
        email_addresses: vec![Email(user.email)],
    })
    .map_err(|e| FieldError::new(e, graphql_value!({"internal_error": "notimplemented"})))?;

    Ok(UserResponse { id: id.0.into() })
}

pub fn user(repo: &UserRepository, id: ID) -> FieldResult<Option<User>> {
    let id = id.parse().map(UserID).map_err(|e| {
        FieldError::new(
            e,
            graphql_value!({"internal_error": "failed to convert to UUID"}),
        )
    })?;
    Ok(repo
        .find_user(&id.0)
        .map_err(|e| FieldError::new(e, graphql_value!({"internal_error": "failed to find user"})))?
        .map(|data| User {
            id: UserID(data.id.0),
        }))
}
