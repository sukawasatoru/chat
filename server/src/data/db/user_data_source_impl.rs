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

use std::io::{prelude::*, BufReader, BufWriter};
use std::path::PathBuf;

use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::data::db::user_data_source::{UserDataSource, UserEntity};
use crate::data::db::util::{convert_to_version, convert_to_version_code};
use crate::model::version::Version;
use crate::prelude::{Fallible, OkOrErr};

const FLUSH_CODE: u16 = 0;

#[derive(Deserialize, Serialize)]
struct UserTable {
    #[serde(rename = "version-code")]
    version_code: u64,
    users: Vec<UserEntity>,
}

impl UserTable {
    fn new() -> Fallible<Self> {
        let (version, flush_code) = generate_latest_database_version()?;

        Ok(Self {
            version_code: convert_to_version_code(&version, flush_code),
            users: Default::default(),
        })
    }
}

pub struct UserDataSourceImpl {
    database_path: PathBuf,
}

impl UserDataSourceImpl {
    pub fn create<T: Into<PathBuf>>(database_path: T) -> Fallible<Self> {
        let data_source = Self {
            database_path: crate::util::get_user_database_file_path(database_path.into()),
        };

        if !data_source.database_path.exists() {
            info!("create dir");
            let parent = data_source.database_path.parent().ok_or_err()?;
            std::fs::create_dir_all(parent)?;

            info!("create file");
            let file = std::fs::File::create(&data_source.database_path)?;
            let mut writer = BufWriter::new(file);
            let table = UserTable::new()?;
            writer.write_all(&toml::to_vec(&table)?)?;
        }

        let version = data_source.database_version()?;
        let latest_version = generate_latest_database_version()?;
        if version != latest_version {
            failure::bail!(
                "need to update database. current: {:?}, expect: {:?}",
                version,
                latest_version
            );
        }

        Ok(data_source)
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

    fn retrieve(&self) -> Fallible<UserTable> {
        let file = std::fs::File::open(&self.database_path)?;
        let mut reader = BufReader::new(file);
        let mut file_string = String::new();
        reader.read_to_string(&mut file_string)?;
        Ok(toml::from_str(&file_string)?)
    }
}

impl UserDataSource for UserDataSourceImpl {
    fn find_user(&self, id: &str) -> Fallible<Option<UserEntity>> {
        Ok(self
            .retrieve()?
            .users
            .into_iter()
            .find(|data| data.id == id))
    }

    fn find_user_by_emails(&self, emails: &[&str]) -> Fallible<Vec<UserEntity>> {
        Ok(self
            .retrieve()?
            .users
            .into_iter()
            .filter(|data| {
                data.email_addresses
                    .iter()
                    .any(|email| emails.contains(&&**email))
            })
            .collect())
    }

    fn save_user(&self, user: UserEntity) -> Fallible<()> {
        let mut table = self.retrieve()?;
        table.users.push(user);
        let file = std::fs::File::create(&self.database_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&toml::to_vec(&table)?)?;

        Ok(())
    }
}

pub fn generate_latest_database_version() -> Fallible<(Version, u16)> {
    Ok((env!("CARGO_PKG_VERSION").parse()?, FLUSH_CODE))
}
