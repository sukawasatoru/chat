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
use crate::data::db::entity::dev_flex_chat_entity::{
    ChannelEntity, ChannelID, CommentEntity, CommentID,
};
use crate::model::juniper_object::OrderDirection;
use crate::model::version::Version;
use crate::prelude::*;

pub struct DevFlexChatRepository {
    database: DevFlexChatDatabase,
}

impl DevFlexChatRepository {
    pub fn prepare<T: Into<PathBuf>>(database_path: T) -> Fallible<Self> {
        let database_path = crate::util::get_chat_database_file_path(database_path.into());

        Ok(Self {
            database: DevFlexChatDatabase::create(database_path)?,
        })
    }

    pub fn channel_long_polling(&self) -> Fallible<ChannelEntity> {
        self.database.channel_long_polling()
    }

    pub fn channels(&self) -> Fallible<Vec<ChannelEntity>> {
        self.database.channels_created_asc()
    }

    pub fn database_version(&self) -> Fallible<(Version, u16)> {
        self.database.database_version()
    }

    pub fn find_channel<T: AsRef<ChannelID>>(&self, id: T) -> Fallible<Option<ChannelEntity>> {
        self.database.find_channel(id)
    }

    pub fn retrieve_channel_after_long_polling<T: AsRef<ChannelID>>(
        &self,
        id: T,
        order_direction: &OrderDirection,
    ) -> Fallible<Vec<ChannelEntity>> {
        self.database
            .channels_after_long_polling(id.as_ref(), order_direction)
    }

    pub fn retrieve_first(
        &self,
        channel_id: &ChannelID,
        count: u32,
        order_direction: &OrderDirection,
    ) -> Fallible<Vec<CommentEntity>> {
        match order_direction {
            OrderDirection::ASC => self
                .database
                .retrieve_first_created_at_asc(channel_id, count),
            OrderDirection::DESC => self
                .database
                .retrieve_first_created_at_desc(channel_id, count),
        }
    }

    pub fn retrieve_after_long_polling(
        &self,
        channel_id: &ChannelID,
        id: &CommentID,
        order_direction: &OrderDirection,
    ) -> Fallible<Vec<CommentEntity>> {
        self.database
            .retrieve_after_long_polling(channel_id, id, order_direction)
    }

    pub fn long_polling(&self, channel_id: &ChannelID) -> Fallible<CommentEntity> {
        self.database.long_polling(channel_id)
    }

    pub fn save_channel<T: Into<ChannelEntity>>(&self, entity: T) -> Fallible<()> {
        self.database.save_channel(entity)
    }

    pub fn save_comment<T: Into<CommentEntity>>(&self, comment: T) -> Fallible<()> {
        self.database.save_comment(comment)
    }
}
