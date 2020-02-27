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

use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::PathBuf;

use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::data::db::entity::dev_flex_chat_entity::CommentEntity;
use crate::prelude::*;

pub struct DevFlexChatDatabase {
    database_path: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct DevFlexChatTable {
    comments: Vec<CommentEntity>,
}

impl DevFlexChatDatabase {
    pub fn new<T: Into<PathBuf>>(database_path: T) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }

    pub fn save_comment<T: Into<CommentEntity>>(&self, entity: T) -> Fallible<()> {
        let mut table = self.retrieve()?;
        table.comments.push(entity.into());
        let file = std::fs::File::create(&self.database_path)?;
        let mut writer = BufWriter::new(file);
        Ok(writer.write_all(&toml::to_vec(&table)?)?)
    }

    pub fn retrieve_after(&self, id: uuid::Uuid) -> Fallible<Vec<CommentEntity>> {
        let table = self.retrieve()?;
        let mut found = false;
        let mut ret = vec![];
        for comment in table.comments {
            if !found && comment.id == id {
                found = true;
            }

            if found {
                ret.push(comment);
                continue;
            }
        }
        Ok(ret)
    }

    pub fn retrieve_first_created_at_desc(&self, count: u32) -> Fallible<Vec<CommentEntity>> {
        let table = self.retrieve()?;
        let mut comments = table.comments;
        let mut ret = vec![];
        for _ in 0..count {
            match comments.pop() {
                Some(data) => ret.push(data),
                None => return Ok(ret)
            }
        }
        Ok(ret)
    }

    pub fn retrieve_all(&self) -> Fallible<Vec<CommentEntity>> {
        let table = self.retrieve()?;
        Ok(table.comments)
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
