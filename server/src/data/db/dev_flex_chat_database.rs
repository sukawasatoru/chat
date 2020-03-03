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

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::data::db::entity::dev_flex_chat_entity::CommentEntity;
use crate::prelude::*;

pub struct DevFlexChatDatabase {
    database_path: PathBuf,
    comment_senders: Mutex<Vec<Sender<CommentEntity>>>,
}

#[derive(Deserialize, Serialize)]
struct DevFlexChatTable {
    comments: Vec<CommentEntity>,
}

impl DevFlexChatDatabase {
    pub fn new<T: Into<PathBuf>>(database_path: T) -> Self {
        Self {
            database_path: database_path.into(),
            comment_senders: Default::default(),
        }
    }

    pub fn save_comment<T: Into<CommentEntity>>(&self, entity: T) -> Fallible<()> {
        let entity = entity.into();
        let mut table = self.retrieve()?;
        table.comments.push(entity.clone());
        let file = std::fs::File::create(&self.database_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&toml::to_vec(&table)?)?;

        match self.comment_senders.lock() {
            Ok(mut senders) => {
                for tx in senders.iter() {
                    tx.send(entity.clone())?;
                }
                senders.clear();
                Ok(())
            }
            Err(e) => failure::bail!("failed to send entity: {:?}", e),
        }
    }

    pub fn retrieve_after(&self, id: uuid::Uuid) -> Fallible<Vec<CommentEntity>> {
        let table = self.retrieve()?;
        let mut found = false;
        let mut ret = vec![];
        for comment in table.comments {
            if !found && comment.id == id {
                found = true;
                continue;
            }

            if found {
                ret.push(comment);
                continue;
            }
        }
        if found {
            Ok(ret)
        } else {
            failure::bail!("id not found: {:?}", id)
        }
    }

    pub fn retrieve_after_long_polling(&self, id: uuid::Uuid) -> Fallible<Vec<CommentEntity>> {
        let mut ret_retrieve_after = self.retrieve_after(id)?;

        if !ret_retrieve_after.is_empty() {
            return Ok(ret_retrieve_after);
        }

        Ok(vec![self.long_polling()?])
    }

    pub fn long_polling(&self) -> Fallible<CommentEntity> {
        let (tx, rx) = channel();
        match self.comment_senders.lock() {
            Ok(mut senders) => {
                senders.push(tx);
            }
            Err(e) => failure::bail!("failed to long polling: {:?}", e),
        }

        Ok(rx.recv()?)
    }

    pub fn retrieve_first_created_at_desc(&self, count: u32) -> Fallible<Vec<CommentEntity>> {
        let table = self.retrieve()?;
        let mut comments = table.comments;
        let mut ret = vec![];
        for _ in 0..count {
            match comments.pop() {
                Some(data) => ret.push(data),
                None => return Ok(ret),
            }
        }
        Ok(ret)
    }

    fn retrieve(&self) -> Fallible<DevFlexChatTable> {
        if !self.database_path.exists() {
            info!("create dir");
            let parent = self.database_path.parent().ok_or_err()?;
            std::fs::create_dir_all(parent)?;

            info!("create file");
            let file = std::fs::File::create(&self.database_path)?;
            let mut writer = BufWriter::new(file);
            let database = DevFlexChatTable {
                comments: Default::default(),
            };
            writer.write_all(&toml::to_vec(&database)?)?;
        }

        let file = std::fs::File::open(&self.database_path)?;
        let mut reader = BufReader::new(file);
        let mut file_string = String::new();
        reader.read_to_string(&mut file_string)?;
        Ok(toml::from_str(&file_string)?)
    }
}
