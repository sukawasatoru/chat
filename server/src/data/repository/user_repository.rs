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

use crate::data::db::user_data_source::{UserDataSource, UserEntity};
use crate::model::{Email, User, UserID};
use crate::prelude::*;

pub struct UserRepository {
    data_source: Box<dyn UserDataSource>,
}

impl UserRepository {
    pub fn prepare(data_source: Box<dyn UserDataSource>) -> Fallible<Self> {
        Ok(Self { data_source })
    }

    pub fn find_user(&self, id: &str) -> Fallible<Option<User>> {
        Ok(self.data_source.find_user(id)?.map(|entity| User {
            id: UserID(entity.id),
            display_name: entity.display_name,
            email_addresses: entity.email_addresses.into_iter().map(Email).collect(),
        }))
    }

    pub fn find_user_by_emails(&self, emails: &[Email]) -> Fallible<Vec<User>> {
        Ok(self
            .data_source
            .find_user_by_emails(
                &emails
                    .iter()
                    .map(|data| data.0.as_str())
                    .collect::<Vec<_>>(),
            )?
            .into_iter()
            .map(|entity| User {
                id: UserID(entity.id),
                display_name: entity.display_name,
                email_addresses: entity.email_addresses.into_iter().map(Email).collect(),
            })
            .collect())
    }

    pub fn save_user(&self, user: User) -> Fallible<()> {
        self.data_source.save_user(UserEntity {
            id: user.id.0,
            display_name: user.display_name,
            email_addresses: user
                .email_addresses
                .into_iter()
                .map(|data| data.0)
                .collect(),
        })
    }
}
