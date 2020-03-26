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

use crate::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserEntity {
    pub id: String,
    pub display_name: String,
    pub email_addresses: Vec<String>,
}

pub trait UserDataSource: Send + Sync {
    fn find_user(&self, id: &str) -> Fallible<Option<UserEntity>>;

    fn find_user_by_emails(&self, emails: &[&str]) -> Fallible<Vec<UserEntity>>;

    fn save_user(&self, user: UserEntity) -> Fallible<()>;
}
