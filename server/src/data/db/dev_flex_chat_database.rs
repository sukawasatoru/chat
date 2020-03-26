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

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::data::db::entity::dev_flex_chat_entity::{
    ChannelEntity, ChannelID, CommentEntity, CommentID,
};
use crate::data::db::util::{convert_to_version, convert_to_version_code};
use crate::model::juniper_object::OrderDirection;
use crate::model::version::Version;
use crate::prelude::*;

// for force update.
const FLUSH_CODE: u16 = 0;

pub struct DevFlexChatDatabase {
    database_path: PathBuf,
    channel_senders: Mutex<Vec<Sender<ChannelEntity>>>,
    comment_senders: Mutex<HashMap<ChannelID, Vec<Sender<CommentEntity>>>>,
}

#[derive(Deserialize, Serialize)]
struct DevFlexChatTable {
    #[serde(rename = "version-code")]
    version_code: u64,
    comments: Vec<CommentEntity>,
    channels: Vec<ChannelEntity>,
}

impl DevFlexChatTable {
    fn new() -> Fallible<Self> {
        let (version, flush_code) = generate_latest_database_version()?;
        Ok(Self {
            version_code: convert_to_version_code(&version, flush_code),
            comments: Default::default(),
            channels: Default::default(),
        })
    }
}

impl DevFlexChatDatabase {
    pub fn create<T: Into<PathBuf>>(database_path: T) -> Fallible<Self> {
        let db = Self {
            database_path: database_path.into(),
            channel_senders: Default::default(),
            comment_senders: Default::default(),
        };

        if !db.database_path.exists() {
            info!("create dir");
            let parent = db.database_path.parent().ok_or_err()?;
            std::fs::create_dir_all(parent)?;

            info!("create file");
            let file = std::fs::File::create(&db.database_path)?;
            let mut writer = BufWriter::new(file);
            let database = DevFlexChatTable::new()?;
            writer.write_all(&toml::to_vec(&database)?)?;
        }

        let version = db.database_version()?;
        let latest_version = generate_latest_database_version()?;
        if version != latest_version {
            failure::bail!(
                "need to update database. current: {:?}, expect: {:?}",
                version,
                latest_version
            );
        }

        Ok(db)
    }

    pub fn channel_long_polling(&self) -> Fallible<ChannelEntity> {
        let (tx, rx) = channel();
        match self.channel_senders.lock() {
            Ok(mut senders) => senders.push(tx),
            Err(e) => failure::bail!("failed to long polling: {:?}", e),
        }

        Ok(rx.recv()?)
    }

    pub fn channels_after_created_asc(
        &self,
        channel_id: &ChannelID,
    ) -> Fallible<Vec<ChannelEntity>> {
        let channels = self.channels_created_asc()?;
        for (index, channel) in channels.iter().enumerate() {
            if &channel.id == channel_id {
                return Ok(channels[index + 1..].to_vec());
            }
        }

        failure::bail!("id not found: {:?}", channel_id)
    }

    pub fn channels_after_long_polling(
        &self,
        channel_id: &ChannelID,
        _order_direction: &OrderDirection,
    ) -> Fallible<Vec<ChannelEntity>> {
        // TODO: use order.

        let channels = self.channels_after_created_asc(channel_id)?;
        if !channels.is_empty() {
            return Ok(channels);
        }

        Ok(vec![self.channel_long_polling()?])
    }

    pub fn channels_created_asc(&self) -> Fallible<Vec<ChannelEntity>> {
        Ok(self.retrieve()?.channels)
    }

    //noinspection DuplicatedCode
    pub fn database_version(&self) -> Fallible<(Version, u16)> {
        let file = std::fs::File::open(&self.database_path)?;
        let mut reader = BufReader::new(file);
        let mut file_string = String::new();
        reader.read_to_string(&mut file_string)?;
        match toml::from_str::<toml::Value>(&file_string)?.get("version-code") {
            Some(data) => Ok(convert_to_version(data.as_integer().ok_or_err()? as u64)),
            None => failure::bail!("failed to retrieve version-code"),
        }
    }

    pub fn find_channel<T: AsRef<ChannelID>>(
        &self,
        channel_id: T,
    ) -> Fallible<Option<ChannelEntity>> {
        for channel in self.retrieve()?.channels {
            if channel.id == *channel_id.as_ref() {
                return Ok(Some(channel));
            }
        }
        Ok(None)
    }

    pub fn save_channel<T: Into<ChannelEntity>>(&self, entity: T) -> Fallible<()> {
        let entity = entity.into();
        let mut table = self.retrieve()?;
        table.channels.push(entity.clone());
        let file = std::fs::File::create(&self.database_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&toml::to_vec(&table)?)?;

        match self.channel_senders.lock() {
            Ok(mut senders) => {
                for tx in senders.iter() {
                    tx.send(entity.clone())?;
                }
                senders.clear();
            }
            Err(e) => failure::bail!("failed to send entity: {:?}", e),
        }

        Ok(())
    }

    pub fn save_comment<T: Into<CommentEntity>>(&self, entity: T) -> Fallible<()> {
        let entity = entity.into();

        if self.find_channel(&entity.channel_id)?.is_none() {
            failure::bail!("channel not found")
        }

        let mut table = self.retrieve()?;
        table.comments.push(entity.clone());
        let file = std::fs::File::create(&self.database_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&toml::to_vec(&table)?)?;

        match self.comment_senders.lock() {
            Ok(mut senders_map) => {
                let senders = match senders_map.get_mut(&entity.channel_id) {
                    Some(data) => data,
                    None => return Ok(()),
                };

                for tx in senders.iter() {
                    tx.send(entity.clone())?;
                }
                senders.clear();
                Ok(())
            }
            Err(e) => failure::bail!("failed to send entity: {:?}", e),
        }
    }

    pub fn retrieve_after_created_asc(
        &self,
        channel_id: &ChannelID,
        id: &CommentID,
    ) -> Fallible<Vec<CommentEntity>> {
        let comments = self.retrieve_comments(channel_id)?;
        for (index, comment) in comments.iter().enumerate() {
            if &comment.id == id {
                return Ok(comments[index + 1..comments.len()].to_vec());
            }
        }

        failure::bail!("id not found: {:?}", id)
    }

    pub fn retrieve_after_created_desc(
        &self,
        channel_id: &ChannelID,
        id: &CommentID,
    ) -> Fallible<Vec<CommentEntity>> {
        let mut comments = self.retrieve_after_created_asc(channel_id, id)?;
        comments.reverse();
        Ok(comments)
    }

    pub fn retrieve_after_long_polling(
        &self,
        channel_id: &ChannelID,
        id: &CommentID,
        order_direction: &OrderDirection,
    ) -> Fallible<Vec<CommentEntity>> {
        let ret_retrieve_after = match order_direction {
            OrderDirection::ASC => self.retrieve_after_created_asc(channel_id, id)?,
            OrderDirection::DESC => self.retrieve_after_created_desc(channel_id, id)?,
        };

        if !ret_retrieve_after.is_empty() {
            return Ok(ret_retrieve_after);
        }

        Ok(vec![self.long_polling(channel_id)?])
    }

    pub fn long_polling(&self, channel_id: &ChannelID) -> Fallible<CommentEntity> {
        let (tx, rx) = channel();
        match self.comment_senders.lock() {
            Ok(mut senders_map) => {
                let senders = match senders_map.get_mut(&channel_id) {
                    Some(data) => data,
                    None => {
                        senders_map.insert(channel_id.clone(), vec![]);
                        senders_map.get_mut(channel_id).ok_or_err()?
                    }
                };
                senders.push(tx);
            }
            Err(e) => failure::bail!("failed to long polling: {:?}", e),
        }

        Ok(rx.recv()?)
    }

    pub fn retrieve_first_created_at_asc(
        &self,
        channel_id: &ChannelID,
        count: u32,
    ) -> Fallible<Vec<CommentEntity>> {
        let comments = self.retrieve_comments(channel_id)?;
        let range_start = if comments.len() < count as usize {
            0
        } else {
            comments.len() - count as usize
        };
        Ok(comments[range_start..comments.len()].to_vec())
    }

    pub fn retrieve_first_created_at_desc(
        &self,
        channel_id: &ChannelID,
        count: u32,
    ) -> Fallible<Vec<CommentEntity>> {
        let mut comments = self.retrieve_first_created_at_asc(channel_id, count)?;
        comments.reverse();
        Ok(comments)
    }

    fn retrieve(&self) -> Fallible<DevFlexChatTable> {
        let file = std::fs::File::open(&self.database_path)?;
        let mut reader = BufReader::new(file);
        let mut file_string = String::new();
        reader.read_to_string(&mut file_string)?;
        Ok(toml::from_str(&file_string)?)
    }

    fn retrieve_comments(&self, channel_id: &ChannelID) -> Fallible<Vec<CommentEntity>> {
        Ok(self
            .retrieve()?
            .comments
            .into_iter()
            .filter(|data| data.channel_id == *channel_id)
            .collect())
    }
}

pub fn generate_latest_database_version() -> Fallible<(Version, u16)> {
    Ok((env!("CARGO_PKG_VERSION").parse()?, FLUSH_CODE))
}
