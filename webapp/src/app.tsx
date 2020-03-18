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
import {
    CommandBar,
    Fabric,
    ICommandBarItemProps,
    INavLink,
    INavLinkGroup,
    initializeIcons,
    List,
    mergeStyles,
    Nav,
    Panel,
    PanelType,
    Stack,
    Text,
    TextField,
} from 'office-ui-fabric-react';
import 'office-ui-fabric-react/dist/css/fabric.min.css'
import {
    CSSProperties,
    default as React,
    FunctionComponentElement,
    KeyboardEvent,
    useCallback,
    useEffect,
    useMemo,
    useRef,
    useState,
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
    overflowX: 'hidden',
    overflowY: 'auto',
});

const calcWindowSize = (): Partial<CSSProperties> => ({
    position: 'absolute',
    top: 0,
    bottom: window.outerHeight - window.innerHeight,
    left: 0,
    right: window.outerWidth - window.innerWidth,
});

const useChannels = (repo: ChatRepository) => {
    const [allChannels, setAllChannels] = useState<ChatChannel[]>([]);
    useEffect(() => {
        const retryCounter = new RetryCounter();
        let timeoutHandle: number | undefined = undefined;
        const abortController = new AbortController();

        const polling = async (channelID: ChannelID): Promise<void> => {
            try {
                const data = await repo.retrieveChannelsWithLongPolling(channelID);
                retryCounter.reset();
                setAllChannels(prev => prev.concat(data));
                window.setTimeout(() => polling(data[data.length - 1].channelID), 0);
            } catch (e) {
                if (e.name === 'AbortError') {
                    console.log(`abort retrieveChannelsWithLongPolling`);
                    return;
                }
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveChannelsWithLongPolling. retry: ${timeout}ms, reason: ${e}`);
                window.setTimeout(() => polling(channelID), timeout);
            }
        };

        const task = async (): Promise<void> => {
            try {
                const channels = await repo.retrieveChannels(abortController.signal);
                retryCounter.reset();
                setAllChannels(channels);

                if (channels.length === 0) {
                    console.log(`TODO`);
                    return;
                }

                // noinspection ES6MissingAwait
                polling(channels[channels.length - 1].channelID);
            } catch (e) {
                if (e.name == 'AbortError') {
                    console.log(`abort retrieveChannels`);
                    return;
                }
                const timeout = retryCounter.timeoutMilliSec();
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
    return allChannels;
};

type useCreateChannelRet = {
    commandItems: ICommandBarItemProps[];
    dismissCreateChannelPanel: () => void;
    isOpenCreateChannelPanel: boolean;
    onCreateChannelNameChanged: (ev: any, newValue?: string) => void;
    onCreateChannelNameKeyDown: (ev: KeyboardEvent) => void;
};

const useCreateChannel = (chat: ChatRepository, onAddChannel?: (channel: ChatChannel) => void): useCreateChannelRet => {
    const [isOpenCreateChannelPanel, setIsOpenCreateChannelPanel] = useState(false);
    const openCreateChannelPanel = useCallback(() => setIsOpenCreateChannelPanel(true), [setIsOpenCreateChannelPanel]);

    const dismissCreateChannelPanel = useCallback(() => setIsOpenCreateChannelPanel(false), [setIsOpenCreateChannelPanel]);

    const commandItems: ICommandBarItemProps[] = [{
        key: 'menu',
        text: 'menu',
        iconProps: {
            iconName: 'GlobalNavButton',
        },
        subMenuProps: {
            items: [
                {
                    key: 'create-channel',
                    text: 'Create Channel',
                    onClick: openCreateChannelPanel,
                },
            ]
        }
    }];

    const [channelName, setChannelName] = useState('');
    const onCreateChannelNameChanged = useCallback((ev: any, newValue?: string): void => {
        setChannelName(newValue ? newValue : '');
    }, [setChannelName]);

    const onCreateChannelNameKeyDown = useCallback((ev: KeyboardEvent) => {
        if (ev.key !== 'Enter') {
            return;
        }
        ev.stopPropagation();
        if (channelName.length === 0) {
            return;
        }
        const task = async (): Promise<void> => {
            try {
                const channel = await chat.addChannel(channelName);
                if (onAddChannel) {
                    onAddChannel(channel);
                }
                dismissCreateChannelPanel();
            } catch (e) {
                console.log(`failed to add channel: ${e}`);
            }
        };
        // noinspection JSIgnoredPromiseFromCall
        task();
    }, [chat, channelName, dismissCreateChannelPanel, onAddChannel]);

    return {
        commandItems,
        dismissCreateChannelPanel,
        isOpenCreateChannelPanel,
        onCreateChannelNameChanged,
        onCreateChannelNameKeyDown,
    };
};

const useWindowSize = () => {
    const createState = () => ({
        outerWidth: window.outerWidth,
        outerHeight: window.outerHeight,
        innerWidth: window.innerWidth,
        innerHeight: window.innerHeight,
    });
    const [windowSize, setWindowSize] = useState(createState());

    useEffect(() => {
        const cb = () => setWindowSize(createState());
        window.addEventListener('resize', cb);
        return () => window.removeEventListener('resize', cb);
    }, []);
    return windowSize;
};

const App = (): FunctionComponentElement<unknown> => {
    const chat = useMemo(() => new ChatRepository(new ChatDataSourceImpl(process.env.GRAPHQL_ENDPOINT)), []);
    const allChannels = useChannels(chat);
    const [currentChannel, setCurrentChannel] = useState<ChatChannel>();
    const [comments, setComments] = useState<ChatComment[]>([]);
    const initialUserName = localStorage.getItem('userName');
    const [userName, setUserName] = useState(initialUserName ? initialUserName : '');
    const [message, setMessage] = useState('');
    const windowSize = useWindowSize();
    const [requestScrollCommentListToBottom, setRequestScrollCommentListToBottom] = useState(false);
    const [addedChannel, setAddedChannel] = useState<ChatChannel>();
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
                if (e.name === 'AbortError') {
                    console.log(`abort retrieveCommentsWithLongPolling`);
                    return;
                }
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

                // noinspection ES6MissingAwait
                polling(currentChannel.channelID, latestID);
            } catch (e) {
                if (e.name == 'AbortError') {
                    console.log(`abort retrieveComments`);
                    return;
                }
                const timeout = retryCounter.timeoutMilliSec();
                console.log(`failed to execute retrieveComments. retry: ${timeout}ms, reason: ${e}`);
                window.setTimeout(task, timeout);
                return;
            }
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
    }, [chat, currentChannel, setComments]);

    useEffect(() => {
        if (allChannels.length == 0) {
            console.log(`TODO:`);
            return;
        }

        setCurrentChannel((prev) => {
            if (!prev) {
                return allChannels[0];
            }

            let ch = addedChannel;
            if (ch) {
                let id = ch.channelID;
                let found = allChannels.find((data) => data.channelID === id);

                if (found) {
                    setAddedChannel(undefined);
                    return found;
                }
            }

            return prev;
        });
    }, [addedChannel, allChannels, setCurrentChannel, setAddedChannel]);

    useEffect(() => localStorage.setItem("userName", userName), [userName]);
    const noMessageKeyDown = useCallback((ev: KeyboardEvent) => {
        if (ev.key === 'Enter') {
            ev.stopPropagation();

            if (message.length === 0) {
                return;
            }
            onSendClick();
        }
    }, [onSendClick, message]);

    const channelListItem = useMemo<INavLinkGroup[]>(() => [{
        links: allChannels.map((data) => ({
            name: data.name,
            url: '',
            key: data.channelID,
        }))
    }], [allChannels]);

    const onClickNav = useCallback((ev?: React.MouseEvent<HTMLElement>, item?: INavLink): void => {
        if (!item) {
            return;
        }

        const channel = allChannels.find((data) => data.channelID == item.key);
        if (channel) {
            setCurrentChannel(channel);
        }
    }, [allChannels, setCurrentChannel]);

    const onAddChannel = useCallback((channel: ChatChannel) => {
        setAddedChannel(channel);
    }, [setAddedChannel]);

    const {
        commandItems,
        dismissCreateChannelPanel,
        isOpenCreateChannelPanel,
        onCreateChannelNameChanged,
        onCreateChannelNameKeyDown,
    } = useCreateChannel(chat, onAddChannel);

    return (
        <Fabric>
            <div className='ms-Grid' dir='ltr' style={{
                width: windowSize.innerWidth,
                height: windowSize.innerHeight
            }}>
                <div className='ms-Grid-row'>
                    <div className='ms-Grid-col ms-hiddenLgDown ms-xl2'>
                        <Nav groups={channelListItem} onLinkClick={onClickNav} selectedKey={currentChannel?.channelID}
                             styles={{root: {height: windowSize.innerHeight}}}/>
                    </div>
                    <div className='ms-Grid-col ms-lg12 ms-xl10' style={{padding: 0}}>
                        <div className={listContainerClass} data-is-scrollable="true"
                             style={{width: '100%', height: `calc(${windowSize.innerHeight}px - 9rem`}}>
                            <List ref={refCommentList} getKey={getCommentKey} items={comments}
                                  onRenderCell={renderCell}/>
                        </div>
                        <div style={{height: '9rem'}}>
                            <TextField style={{fontSize: '16px'}} label={'name'} defaultValue={userName}
                                       onChange={(e: any, value?: string) => setUserName(value ? value : '')}/>
                            <div style={{display: 'flex'}}>
                                <div style={{flex: '0 1 auto', alignSelf: 'center'}}>
                                    <CommandBar items={commandItems}/>
                                </div>
                                <div style={{flex: '1 1 auto'}}>
                                    <TextField style={{fontSize: '16px'}} label={'message'}
                                               onChange={(e: any, value?: string) => setMessage(value ? value : '')}
                                               onKeyDown={noMessageKeyDown} value={message}
                                               description={'return to send'}/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <Panel dir='ltr'
                   closeButtonAriaLabel='Close' headerText='Create Channel' isOpen={isOpenCreateChannelPanel}
                   onDismiss={dismissCreateChannelPanel} type={PanelType.smallFluid}>
                Hello
                <TextField onChange={onCreateChannelNameChanged} onKeyDown={onCreateChannelNameKeyDown}
                           style={{fontSize: '16px'}}/>
            </Panel>
        </Fabric>
    );
};

export default App;
