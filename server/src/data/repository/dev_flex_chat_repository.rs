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

pub struct DevFlexChatRepository {
    database_path: PathBuf,
}

impl Default for DevFlexChatRepository {
    fn default() -> Self {
        Self {
            database_path: PathBuf::new(),
        }
    }
}

impl DevFlexChatRepository {
    pub fn new<T: Into<PathBuf>>(database_path: T) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }
}
