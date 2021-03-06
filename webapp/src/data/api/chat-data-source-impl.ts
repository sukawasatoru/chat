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
    AddChannelResponse,
    AddCommentResponse,
    ChannelsResponse,
    ChannelsWithLongPollingResponse,
    ChatDataSource,
    CommentsResponse,
    RetrieveCommentsWithLongPollingResponse
} from '@/data/api/chat-data-source';

import {ChannelID} from '@/model/chat-models';

// TODO: use apollo.
const graphQLRequest = (url: string, query: string, variables: object, abortSignal?: AbortSignal): Promise<Response> => {
    return fetch(url, {
        headers: {
            // for avoid OPTIONS method.
            // 'content-type': 'application/json; charset=utf-8',
            accept: 'application/json',
        },
        method: 'POST',
        mode: 'cors',
        body: JSON.stringify({query, variables}),
        signal: abortSignal,
    });
};

export class ChatDataSourceImpl implements ChatDataSource {
    private readonly graphQLURL: string;

    constructor(graphQLURL: string) {
        this.graphQLURL = graphQLURL;
    }

    async addChannel(channelName: string, abortSignal?: AbortSignal): Promise<AddChannelResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
mutation($name: String!) {
  addChannel(channel: { name: $name }) {
    id
    name
  }
}`, {name: channelName}, abortSignal);

        if (!response.ok) {
            throw new Error(`failed to add channel: ${response.body}`);
        }

        return response.json();
    }

    async addComment(channelID: ChannelID, userName: string, message: string, abortSignal?: AbortSignal): Promise<AddCommentResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
mutation($channelId: ID!, $name: String!, $message: String!) {
  addComment(
    comment: { channelId: $channelId, name: $name, message: $message }
  ) {
    id
    name
    message
  }
}`, {channelId: channelID, name: userName, message}, abortSignal);

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to send comment: ${response.body}`);
        }

        return response.json();
    }

    async retrieveChannels(abortSignal?: AbortSignal): Promise<ChannelsResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
{
  channels {
    id
    name
  }
}`, {}, abortSignal);

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to retrieve channels`);
        }

        return await response.json();
    }

    async retrieveChannelsWithLongPolling(channelID: ChannelID, abortSignal?: AbortSignal): Promise<ChannelsWithLongPollingResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
query($id: ID!) {
  channelLongPolling(id: $id, orderBy: { direction: ASC }) {
    id
    name
  }
}`, {id: channelID}, abortSignal);

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed long polling for channel`);
        }

        return response.json();
    }


    async retrieveComments(channelID: ChannelID, abortSignal?: AbortSignal): Promise<CommentsResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
query($channelId: ID!, $first:Int!, $direction: OrderDirection!) {
  channel(id: $channelId) {
    comments(first: $first, orderBy: { direction: $direction }) {
      id
      name
      message
    }
  }
}`, {channelId: channelID, first: 100, direction: 'ASC'}, abortSignal);

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to retrieve comments`);
        }

        return await response.json();
    }

    async retrieveCommentsWithLongPolling(channelID: ChannelID, lastID?: string, abortSignal?: AbortSignal): Promise<RetrieveCommentsWithLongPollingResponse> {
        const response = await graphQLRequest(this.graphQLURL, `
query($channelId: ID!, $commentId: ID, $direction: OrderDirection!) {
  channel(id: $channelId) {
    commentsLongPolling(id: $commentId, orderBy: { direction: $direction }) {
      id
      name
      message
    }
  }
}`, {channelId: channelID, commentId: lastID, direction: 'ASC'}, abortSignal);

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to long polling`);
        }
        return await response.json();
    }
}
