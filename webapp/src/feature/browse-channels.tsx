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

import {ChatChannel} from '@/model/chat-models';
import {CompoundButton, List, Panel, PanelType, Stack} from 'office-ui-fabric-react';
import {default as React, FunctionComponentElement, useCallback} from 'react';

type Props = {
    channels: ChatChannel[];
    isOpen: boolean;
    onClickChannel: (channel: ChatChannel) => void;
    onDismiss: () => void;
};

const getChannelKey = (item?: ChatChannel, index?: number): string => item ? item.channelID : 'none';

const useRenderCell = (onClickCb: (channel: ChatChannel) => void): (item?: ChatChannel, index?: number, isScrolling?: boolean) => React.ReactNode => {
    return useCallback((item?: ChatChannel, index?: number, isScrolling?: boolean): React.ReactNode => {
        if (!item) {
            return <div>
                (none)
            </div>;
        }

        return <Stack tokens={{padding: '8px'}}>
            <CompoundButton onClick={() => onClickCb(item)}>
                {item?.name}
            </CompoundButton>
        </Stack>;
    }, [onClickCb]);
};

export const BrowseChannels = (props: Props): FunctionComponentElement<Props> => {
    const renderCell = useRenderCell(props.onClickChannel);

    return <Panel closeButtonAriaLabel='Close' headerText='Browse Channels' isOpen={props.isOpen}
                  onDismiss={props.onDismiss} type={PanelType.smallFluid}>
        <List getKey={getChannelKey} items={props.channels} onRenderCell={renderCell}/>
    </Panel>;
};

export default BrowseChannels;
