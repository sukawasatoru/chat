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

export type CommentsResponse = {
    data: {
        comments: {
            id: string;
            name: string;
            message: string;
        }[];
    };
};

export type RetrieveCommentsWithLongPollingResponse = {
    data: {
        commentsLongPolling: {
            id: string;
            name: string;
            message: string;
        }[];
    };
};

export type AddCommentResponse = {
    data: {
        addComment: {
            id: string,
            name: string,
            message: string,
        };
    };
}


export interface ChatDataSource {
    retrieveComments(): Promise<CommentsResponse>;

    retrieveCommentsWithLongPolling(lastID?: string): Promise<RetrieveCommentsWithLongPollingResponse>;

    addComment(userName: string, message: string): Promise<AddCommentResponse>;
}
