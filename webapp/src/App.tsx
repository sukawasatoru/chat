/*
 * Copyright 2019, 2020 sukawasatoru
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

import {default as React, useCallback, useState} from 'react';
import {Fabric, Icon, initializeIcons, PrimaryButton} from 'office-ui-fabric-react';
import './App.css';

initializeIcons();

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

interface CommentsResponse {
    data: {
        comments: {
            id: string;
            name: string;
            message: string;
        }[];
    };
}

class ChatServer {
    private readonly graphQLURL: string;

    constructor(baseUrl: string) {
        this.graphQLURL = baseUrl;
    }

    public async retrieveComments(): Promise<string[]> {
        const response = await graphQLRequest(this.graphQLURL, `
query($first: Int!){
  comments(first: $first) {
    id
    name
    message
  }
}
`, {
            first: 100
        });

        if (!response.ok) {
            console.log(response);
            throw new Error(`failed to retrieve comments`);
        }

        const payload: CommentsResponse = await response.json();
        return payload.data.comments.map(data => `name: ${data.name}, message: ${data.message}`);
    }
}

const App = () => {
    const chat = new ChatServer(process.env.GRAPHQL_ENDPOINT);
    const [comments, setComments] = useState<string[]>([]);
    const onRetrieveCommentsClicked = useCallback(() => {
        const task = async (): Promise<void> => {
            setComments(await chat.retrieveComments());
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
    }, [chat, setComments]);
    return (
        <Fabric>
            <Icon iconName={'Home'}/>
            Hello
            <br/>
            <PrimaryButton onClick={onRetrieveCommentsClicked}>Retrieve Comment</PrimaryButton>
            {comments.map((data) =>
                // TODO: key
                <div key={Math.random()}>{data}<br/></div>
            )}
        </Fabric>
    );
};

export default App;
