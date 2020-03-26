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

pub fn get_database_path(database_path: Option<PathBuf>) -> PathBuf {
    database_path.unwrap_or_else(|| std::path::Path::new(".").to_owned())
}

pub fn get_chat_database_file_path(database_path: PathBuf) -> PathBuf {
    database_path.join("database.toml")
}

pub fn get_user_database_file_path(database_path: PathBuf) -> PathBuf {
    database_path.join("user.toml")
}
