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

import {ChatDataSource} from '@/data/api/chat-data-source';
import {ChannelID, ChatChannel, ChatComment, CommentID} from '@/model/chat-models';

export class ChatRepository {
    private readonly dataSource: ChatDataSource;

    constructor(dataSource: ChatDataSource) {
        this.dataSource = dataSource;
    }

    async addComment(channelID: ChannelID, userName: string, message: string, abortSignal?: AbortSignal): Promise<ChatComment> {
        const {id, name, message: mes} = (await this.dataSource.addComment(channelID, userName, message, abortSignal))
            .data.addComment;
        return new ChatComment({channelID, commentID: id as CommentID, name, message: mes});
    }

    async retrieveChannels(abortSignal?: AbortSignal): Promise<ChatChannel[]> {
        return (await this.dataSource.retrieveChannels(abortSignal))
            .data.channels.map(({id, name}) => new ChatChannel({channelID: id as ChannelID, name}));
    }

    async retrieveComments(channelID: ChannelID, abortSignal?: AbortSignal): Promise<ChatComment[]> {
        return (await this.dataSource.retrieveComments(channelID, abortSignal))
            .data.channel.comments.map(({id, name, message}) => new ChatComment({
                channelID,
                commentID: id as CommentID,
                name,
                message
            }));
    }

    async retrieveCommentsWithLongPolling(channelID: ChannelID, lastID?: string, abortSignal?: AbortSignal): Promise<ChatComment[]> {
        return (await this.dataSource.retrieveCommentsWithLongPolling(channelID, lastID, abortSignal))
            .data.channel.commentsLongPolling.map(({id, name, message}) => new ChatComment({
                channelID,
                commentID: id as CommentID,
                name,
                message
            }));
    }
}
