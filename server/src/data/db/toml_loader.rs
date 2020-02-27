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

use crate::prelude::*;

pub struct TomlLoader {
    buf: String,
}

impl TomlLoader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load<'a, T>(&'a mut self, path: &std::path::Path) -> Fallible<T>
    where
        T: serde::de::Deserialize<'a>,
    {
        self.buf.clear();
        std::io::BufReader::new(std::fs::File::open(path)?).read_to_string(&mut self.buf)?;
        Ok(toml::from_str::<T>(&self.buf)?)
    }
}

impl Default for TomlLoader {
    fn default() -> Self {
        Self {
            buf: Default::default(),
        }
    }
}
