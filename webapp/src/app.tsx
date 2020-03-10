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

import '@/app.css';
import {ChatDataSourceImpl} from '@/data/api/chat-data-source-impl';
import {ChatRepository} from '@/data/repository/chat-repository';
import {ChannelID, ChatChannel, ChatComment, CommentID} from '@/model/chat-models';
import {Fabric, initializeIcons, List, mergeStyles, Stack, Text, TextField} from 'office-ui-fabric-react';
import {
    CSSProperties,
    default as React,
    FunctionComponentElement,
    KeyboardEvent,
    useCallback,
    useEffect,
    useRef,
    useState,
    useMemo,
} from 'react';

initializeIcons();

document.addEventListener('touchstart', (e) => {
    if (1 < e.touches.length) {
        e.preventDefault();
    }
}, {passive: false});

let lastTouchEnd = 0;
document.addEventListener('touchend', (e) => {
    const currentTouchEnd = Date.now();
    if (currentTouchEnd - lastTouchEnd < 500) {
        e.preventDefault();
    }
    lastTouchEnd = currentTouchEnd;
}, {passive: false});

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
        <Stack tokens={{padding: '8px', maxHeight: '80px'}}>
            <div style={{fontWeight: 'bold'}}>
                {item.name}
            </div>
            <Text variant={'medium'} style={{wordBreak: 'break-word'}}>
                {item.message}
            </Text>
        </Stack>
    );
};

const getCommentKey = (item?: ChatComment, index?: number): string => item ? item.commentID : 'none';

const listContainerClass = mergeStyles({
    overflow: 'scroll',
});

const App = (): FunctionComponentElement<unknown> => {
    const chat = useMemo(() => new ChatRepository(new ChatDataSourceImpl(process.env.GRAPHQL_ENDPOINT)), []);
    const [currentChannel, setCurrentChannel] = useState<ChatChannel>();
    const [comments, setComments] = useState<ChatComment[]>([]);
    const initialUserName = localStorage.getItem('userName');
    const [userName, setUserName] = useState(initialUserName ? initialUserName : '');
    const [message, setMessage] = useState('');
    const [windowSize, setWindowSize] = useState<CSSProperties>({
        width: window.innerWidth,
        height: window.innerHeight,
    });
    const [requestScrollCommentListToBottom, setRequestScrollCommentListToBottom] = useState(false);
    const refCommentList = useRef<List<ChatComment>>(null);

    const onSendClick = useCallback(() => {
        const task = async (): Promise<void> => {
            if (!currentChannel) {
                console.log(`!currentChannel`);
                return;
            }
            try {
                await chat.addComment(currentChannel.channelID, userName, message);
                setMessage('');
                setRequestScrollCommentListToBottom(true);
            } catch (e) {
                console.log(`failed to send comment: ${e}`);
            }
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
    }, [chat, currentChannel, userName, message, setMessage, setRequestScrollCommentListToBottom]);

    useEffect(() => {
        if (!refCommentList.current) {
            return;
        }
        if (requestScrollCommentListToBottom) {
            setRequestScrollCommentListToBottom(false);
            refCommentList.current.scrollToIndex(comments.length - 1);
        }
    }, [comments, requestScrollCommentListToBottom, setRequestScrollCommentListToBottom, refCommentList]);

    useEffect(() => {
        const retryCounter = new RetryCounter();
        let timeoutHandle: number | undefined = undefined;
        let pollingTimeoutHandle: number | undefined = undefined;
        const abortController = new AbortController();

        const polling = async (channelID: ChannelID, commentID?: CommentID): Promise<void> => {
            try {
                const data = await chat.retrieveCommentsWithLongPolling(channelID, commentID, abortController.signal);
                retryCounter.reset();
                setComments(prev => prev.concat(data));
                window.setTimeout(() => polling(channelID, data[data.length - 1].commentID), 0);
            } catch (e) {
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveCommentsWithLongPolling. retry: ${timeout}ms, reason: ${e}`);
                window.setTimeout(() => polling(channelID, commentID), timeout);
            }
        };

        const task = async (): Promise<void> => {
            let latestID: CommentID | undefined = undefined;
            if (!currentChannel) {
                return;
            }

            try {
                const data = await chat.retrieveComments(currentChannel.channelID, abortController.signal);
                retryCounter.reset();
                // TODO: retrieve prev comment when scrolling to up.
                setComments(data);
                if (0 < data.length) {
                    latestID = data[data.length - 1].commentID;
                    if (refCommentList.current) {
                        refCommentList.current.scrollToIndex(data.length - 1);
                    }
                }
            } catch (e) {
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveComments. retry: ${timeout}ms, reason: ${e}`);
                window.setTimeout(task, timeout);
                return;
            }

            // noinspection ES6MissingAwait
            polling(currentChannel.channelID, latestID);
        };
        // noinspection JSIgnoredPromiseFromCall
        task();

        return () => {
            abortController.abort();
            if (timeoutHandle) {
                window.clearTimeout(timeoutHandle);
            }

            if (pollingTimeoutHandle) {
                window.clearTimeout(pollingTimeoutHandle);
            }
        }
    }, [chat, currentChannel, setComments, setCurrentChannel]);

    useEffect(() => {
        const retryCounter = new RetryCounter();
        let timeoutHandle: number | undefined = undefined;
        const abortController = new AbortController();
        const task = async(): Promise<void> => {
            try {
                const channels = await chat.retrieveChannels(abortController.signal);
                retryCounter.reset();
                if (channels.length == 0) {
                    console.log(`TODO:`);
                    return;
                }
                setCurrentChannel(channels[0]);
            } catch (e) {
                const timeout =  retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveChannels. retry: ${timeout}ms`);
                timeoutHandle = window.setTimeout(task, timeout);
            }
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
        return () => {
            abortController.abort();

            if (timeoutHandle !== undefined) {
                window.clearTimeout(timeoutHandle);
            }
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    useEffect(() => localStorage.setItem("userName", userName), [userName]);
    const noMessageKeyDown = useCallback((ev: KeyboardEvent) => {
        if (ev.key == 'Enter') {
            ev.stopPropagation();

            if (message.length == 0) {
                return;
            }
            onSendClick();
        }
    }, [onSendClick, message]);

    useEffect(() => {
        const windowSizeCb = () => setWindowSize({
            position: 'absolute',
            top: 0,
            bottom: window.outerHeight - window.innerHeight,
            left: 0,
            right: window.outerWidth - window.innerWidth,
        });
        window.addEventListener('resize', windowSizeCb);
        return () => window.removeEventListener('resize', windowSizeCb);
    }, []);

    return (
        <Fabric>
            <Stack style={windowSize}>
                <div className={listContainerClass} data-is-scrollable="true">
                    <List ref={refCommentList} getKey={getCommentKey} items={comments} onRenderCell={renderCell}/>
                </div>
                <div style={{width: '100%', alignSelf: 'flex-end'}}>
                    <Stack tokens={{childrenGap: 15}} style={{margin: '16px'}}>
                        <TextField style={{fontSize: '16px'}} label={'name'} defaultValue={userName}
                                   onChange={(e: any, value?: string) => setUserName(value ? value : '')}/>
                        <TextField style={{fontSize: '16px'}} label={'message'}
                                   onChange={(e: any, value?: string) => setMessage(value ? value : '')}
                                   onKeyDown={noMessageKeyDown} value={message} description={'return to send'}/>
                    </Stack>
                </div>
            </Stack>
        </Fabric>
    );
};

export default App;
