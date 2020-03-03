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

import {ChatDataSource} from '../api/chat-data-source';
import {ChatComment} from '../../model/chat-models';


export class ChatRepository {
    private readonly dataSource: ChatDataSource;

    public constructor(dataSource: ChatDataSource) {
        this.dataSource = dataSource;
    }

    public async retrieveComments(): Promise<ChatComment[]> {
        return (await this.dataSource.retrieveComments())
            .data.comments.map(({id, name, message}) => new ChatComment({id, name, message}));
    }

    public async retrieveCommentsWithLongPolling(lastID?: string): Promise<ChatComment[]> {
        return (await this.dataSource.retrieveCommentsWithLongPolling(lastID))
            .data.commentsLongPolling.map(({id, name, message}) => new ChatComment({id, name, message}));
    }

    public async addComment(userName: string, message: string): Promise<ChatComment> {
        const {id, name, message: mes} = (await this.dataSource.addComment(userName, message))
            .data.addComment;
        return new ChatComment({id, name, message: mes});
    }
}
