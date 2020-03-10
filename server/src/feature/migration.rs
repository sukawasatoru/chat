/*
 * Copyright 2019, 2020 sukawasatoru
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
use std::path::{Path, PathBuf};

use log::info;

use crate::data::db::dev_flex_chat_database::{
    convert_to_version, convert_to_version_code, generate_latest_database_version,
};
use crate::data::db::entity::dev_flex_chat_entity::{ChannelEntity, ChannelID};
use crate::prelude::*;

pub fn migration(database_dir: Option<PathBuf>) -> Fallible<()> {
    let (current_version, current_version_flush_code) = generate_latest_database_version()?;
    let current_version_code =
        convert_to_version_code(&current_version, current_version_flush_code);

    info!(
        "app version: {}, flush_code: {}",
        current_version, current_version_flush_code
    );

    let database_file_path = crate::util::get_database_file_path(database_dir);
    let database_version_code = retrieve_database_version_code(&database_file_path)?;
    let (database_version, database_flush_code) = convert_to_version(database_version_code);

    info!(
        "database version: {}, flush_code: {}",
        database_version, database_flush_code
    );

    if current_version_code < database_version_code {
        failure::bail!("need to upgrade the app to migrate")
    }

    let map = [(convert_to_version_code(&[0, 3, 0].into(), 0), migrate_0_3_0)];
    if current_version_code < map[map.len() - 1].0 {
        failure::bail!("need to update a version in Cargo");
    }

    for (target_code, ref f) in &map {
        if database_version_code < *target_code {
            f(&database_file_path)?;
        }
    }

    set_database_version_code(database_file_path, current_version_code)?;
    Ok(())
}

fn retrieve_database_version_code<T: AsRef<Path>>(file_path: T) -> Fallible<u64> {
    match read_file_as_toml_value(file_path)?.get("version-code") {
        Some(data) => Ok(data.as_integer().ok_or_err()? as u64),
        None => Ok(convert_to_version_code(&[0, 1, 0].into(), 0)),
    }
}

fn set_database_version_code<T: AsRef<Path>>(file_path: T, version_code: u64) -> Fallible<()> {
    info!("set database version-code: {}", version_code);

    let mut value = read_file_as_toml_value(file_path.as_ref())?;
    value["version-code"] = toml::Value::Integer(version_code as i64);

    info!("write data");
    let twitter_file_string = toml::to_string(&value)?;
    let twitter_file = std::fs::File::create(file_path)?;
    let mut writer = std::io::BufWriter::new(twitter_file);
    writer.write_all(twitter_file_string.as_bytes())?;

    info!("succeeded to set a version to the database");

    Ok(())
}

fn read_file_as_toml_value<T: AsRef<Path>>(file_path: T) -> Fallible<toml::Value> {
    let file = std::fs::File::open(file_path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    Ok(toml::from_str(&data)?)
}

fn migrate_0_3_0(file_path: &Path) -> Fallible<()> {
    info!("migrate to v0.3.0");

    let mut table = read_file_as_toml_value(file_path)?;
    let table_table = table.as_table_mut().ok_or_err()?;
    table_table.insert(
        "version-code".into(),
        toml::Value::Integer(convert_to_version_code(&[0, 3, 0].into(), 0) as i64),
    );

    let general_channel_uuid = ChannelID(uuid::Uuid::new_v4());

    table_table.insert(
        "channels".into(),
        toml::Value::Array(vec![toml::Value::try_from(ChannelEntity {
            id: general_channel_uuid.clone(),
            name: "General".into(),
        })?]),
    );

    for entry in table["comments"].as_array_mut().ok_or_err()? {
        entry.as_table_mut().ok_or_err()?.insert(
            "channel-id".into(),
            toml::Value::try_from(&general_channel_uuid.0)?,
        );
    }

    info!("write data");
    let twitter_file_string = toml::to_string(&table)?;
    let twitter_file = std::fs::File::create(file_path)?;
    let mut writer = std::io::BufWriter::new(twitter_file);
    writer.write_all(twitter_file_string.as_bytes())?;

    info!("succeeded migrate to v0.3.0");
    Ok(())
}
