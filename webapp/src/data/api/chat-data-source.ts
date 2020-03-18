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

import {ChannelID, ChatChannel} from '@/model/chat-models';

export type AddChannelResponse = {
    data: {
        addChannel: {
            id: string,
            name: string,
        };
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


export type ChannelsResponse = {
    data: {
        channels: {
            id: string;
            name: string;
        }[];
    };
}

export type ChannelsWithLongPollingResponse = {
    data: {
        channelLongPolling: {
            id: string;
            name: string;
        }[];
    };
}

export type CommentsResponse = {
    data: {
        channel: {
            comments: {
                id: string;
                name: string;
                message: string;
            }[];
        };
    };
};

export type RetrieveCommentsWithLongPollingResponse = {
    data: {
        channel: {
            commentsLongPolling: {
                id: string;
                name: string;
                message: string;
            }[];
        };
    };
};

export interface ChatDataSource {
    addChannel(channelName: string, abortSignal?: AbortSignal): Promise<AddChannelResponse>;

    addComment(channelID: ChannelID, userName: string, message: string, abortSignal?: AbortSignal): Promise<AddCommentResponse>;

    retrieveChannels(abortSignal?: AbortSignal): Promise<ChannelsResponse>;

    retrieveChannelsWithLongPolling(channelID: ChannelID, abortSignal?: AbortSignal): Promise<ChannelsWithLongPollingResponse>

    retrieveComments(channelID: ChannelID, abortSignal?: AbortSignal): Promise<CommentsResponse>;

    retrieveCommentsWithLongPolling(channelID: ChannelID, lastID?: string, abortSignal?: AbortSignal): Promise<RetrieveCommentsWithLongPollingResponse>;
}
