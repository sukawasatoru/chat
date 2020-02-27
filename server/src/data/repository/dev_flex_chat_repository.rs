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

use crate::data::db::dev_flex_chat_database::DevFlexChatDatabase;
// TODO: use model.
use crate::data::db::entity::dev_flex_chat_entity::CommentEntity;
use crate::prelude::*;

pub struct DevFlexChatRepository {
    database_path: PathBuf,
    database: DevFlexChatDatabase,
}

impl DevFlexChatRepository {
    pub fn new<T: Into<PathBuf>>(database_path: T) -> Self {
        let database_path = database_path.into();

        Self {
            database_path: database_path.to_owned(),
            database: DevFlexChatDatabase::new(database_path),
        }
    }

    pub fn retrieve_all(&self) -> Fallible<Vec<CommentEntity>> {
        self.database.retrieve_all()
    }

    pub fn retrieve_first(&self, count: u32) -> Fallible<Vec<CommentEntity>> {
        self.database.retrieve_first_created_at_desc(count)
    }

    pub fn save_comment<T: Into<CommentEntity>>(&self, comment: T) -> Fallible<()> {
        self.database.save_comment(comment)
    }
}
