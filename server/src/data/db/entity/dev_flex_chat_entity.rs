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

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommentID(pub uuid::Uuid);

#[derive(Clone, Deserialize, Serialize)]
pub struct CommentEntity {
    pub id: CommentID,

    #[serde(rename = "channel-id")]
    pub channel_id: ChannelID,

    pub name: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelID(pub uuid::Uuid);

impl AsRef<ChannelID> for ChannelID {
    fn as_ref(&self) -> &ChannelID {
        self
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ChannelEntity {
    pub id: ChannelID,
    pub name: String,
}
