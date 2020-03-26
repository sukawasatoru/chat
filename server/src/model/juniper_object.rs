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

use crate::data::repository::dev_flex_chat_repository::DevFlexChatRepository;
use crate::data::repository::user_repository::UserRepository;

pub struct Context {
    pub chat_repo: DevFlexChatRepository,
    pub user_repo: UserRepository,
}

impl juniper::Context for Context {
    // do nothing.
}

impl Context {
    pub fn new(chat_repo: DevFlexChatRepository, user_repo: UserRepository) -> Self {
        Self {
            chat_repo,
            user_repo,
        }
    }
}

#[derive(Debug, juniper::GraphQLEnum)]
pub enum OrderDirection {
    ASC,
    DESC,
}
