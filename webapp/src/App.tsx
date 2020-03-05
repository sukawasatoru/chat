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

import '@/App.css';
import {ChatDataSourceImpl} from '@/data/api/chat-data-source-impl';
import {ChatRepository} from '@/data/repository/chat-repository';
import {ChatComment} from '@/model/chat-models';
import {Fabric, Icon, initializeIcons, List, PrimaryButton, Stack, TextField} from 'office-ui-fabric-react';
import {default as React, FunctionComponentElement, useCallback, useEffect, useState} from 'react';

initializeIcons();

class RetryCounter {
    private readonly step: number[];
    private currentStepIndex: number;

    constructor() {
        this.step = [0, 1, 2, 3, 5, 7, 30, 60];
        this.currentStepIndex = 0;
    }

    public timeoutMilliSec(): number {
        if (this.currentStepIndex + 1 < this.step.length) {
            ++this.currentStepIndex;
        }
        return this.step[this.currentStepIndex] * 1000;
    }

    public reset(): void {
        this.currentStepIndex = 0;
    }
}

const renderCell = (item?: ChatComment, index?: number, isScrolling?: boolean): React.ReactNode => {
    if (!item) {
        return <div>(none)</div>;
    }

    return (
        <div>
            name: {item.name}, {item.message}
        </div>
    );
};

const getCommentKey = (item?: ChatComment, index?: number): string => item ? item.id : 'none';

const App = (): FunctionComponentElement<unknown> => {
    const chat = new ChatRepository(new ChatDataSourceImpl(process.env.GRAPHQL_ENDPOINT));
    const [comments, setComments] = useState<ChatComment[]>([]);
    const initialUserName = localStorage.getItem('userName');
    const [userName, setUserName] = useState(initialUserName ? initialUserName : '');
    const [message, setMessage] = useState('');

    const onRetrieveCommentsClicked = useCallback(() => {
        const task = async (): Promise<void> => {
            setComments(await chat.retrieveComments());
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
    }, [chat, setComments]);

    const onSendClick = useCallback(() => {
        const task = async (): Promise<void> => {
            try {
                await chat.addComment(userName, message);
            } catch (e) {
                console.log(`failed to send comment: ${e}`);
            }
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
    }, [chat, userName, message]);

    useEffect(() => {
        const retryCounter = new RetryCounter();
        const polling = async (id?: string): Promise<void> => {
            try {
                const data = await chat.retrieveCommentsWithLongPolling(id);
                retryCounter.reset();
                setComments(prev => data.concat(prev));
                window.setTimeout(() => polling(data[0].id), 0);
            } catch (e) {
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveCommentsWithLongPolling. retry: ${timeout}ms`);
                window.setTimeout(() => polling(id), timeout);
            }
        };
        const task = async (): Promise<void> => {
            let latestID: string | undefined = undefined;
            try {
                const data = await chat.retrieveComments();
                retryCounter.reset();
                setComments(prev => data.concat(prev));
                if (0 < data.length) {
                    latestID = data[0].id;
                }
            } catch (e) {
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveComments. retry: ${timeout}ms`);
                window.setTimeout(task, timeout);
                return;
            }

            // noinspection ES6MissingAwait
            polling(latestID);
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    useEffect(() => localStorage.setItem("userName", userName), [userName]);

    return (
        <Fabric>
            <Icon iconName={'Home'}/>
            Hello
            <br/>
            <PrimaryButton onClick={onRetrieveCommentsClicked}>Retrieve Comment</PrimaryButton>
            <Stack tokens={{childrenGap: 15}} horizontal>
                <TextField label={'name'} defaultValue={userName}
                           onChange={(e: any, value?: string) => setUserName(value ? value : '')}/>
                <TextField label={'message'}
                           onChange={(e: any, value?: string) => setMessage(value ? value : '')}/>
            </Stack>
            <PrimaryButton onClick={onSendClick}>Send</PrimaryButton>
            <List getKey={getCommentKey} items={comments} onRenderCell={renderCell}/>
        </Fabric>
    );
};

export default App;
