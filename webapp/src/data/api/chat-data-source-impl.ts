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

import {
    AddCommentResponse,
    ChatDataSource,
    CommentsResponse,
    RetrieveCommentsWithLongPollingResponse
} from '@/data/api/chat-data-source';

// TODO: use apollo.
const graphQLRequest = (url: string, query: string, variables: object): Promise<Response> => {
    return fetch(url, {
        headers: {
            // for avoid OPTIONS method.
            // 'content-type': 'application/json; charset=utf-8',
            accept: 'application/json',
        },
        method: 'POST',
        mode: 'cors',
        body: JSON.stringify({query, variables}),
    });
};

export class ChatDataSourceImpl implements ChatDataSource {
    private readonly graphQLURL: string;

    constructor(graphQLURL: string) {
        this.graphQLURL = graphQLURL;
    }

    async addComment(userName: string, message: string): Promise<AddCommentResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
mutation ($name: String!, $message: String!) {
  addComment(comment: {name: $name, message: $message}) {
    id
    name
    message
  }
}
`, {name: userName, message});

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to send comment`);
        }

        return response.json();
    }

    async retrieveComments(): Promise<CommentsResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
query($first: Int!, $direction: OrderDirection!){
  comments(first: $first, orderBy: {direction: $direction}) {
    id
    name
    message
  }
}
`, {first: 100, direction: 'ASC'});

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to retrieve comments`);
        }

        return await response.json();
    }

    async retrieveCommentsWithLongPolling(lastID?: string): Promise<RetrieveCommentsWithLongPollingResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
query ($id: String, $direction: OrderDirection!) {
  commentsLongPolling(id: $id, orderBy: {direction: $direction}) {
    id
    name
    message
  }
}`, {id: lastID ? lastID : null, direction: 'ASC'});

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to long polling`);
        }
        return await response.json();
    }
}
