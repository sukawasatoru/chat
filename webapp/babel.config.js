/*
 * Copyright 2019 sukawasatoru
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

'use strict';

/**
 * @param {ConfigAPI} api
 * @returns {TransformOptions}
 */
module.exports = (api) => {
    const isDev = !api.env('production');
    return {
        presets: [
            ["@babel/preset-env"],
        ],
        plugins: [
            ["@babel/plugin-transform-react-jsx"],
            ["@babel/plugin-transform-react-constant-elements"],
            ["@babel/plugin-transform-react-display-name"],
            isDev && ["@babel/plugin-transform-react-jsx-source"],
            ["@babel/plugin-transform-react-jsx-self"],
            isDev && ["@babel/plugin-transform-react-inline-elements"],
            ["@babel/plugin-syntax-dynamic-import"],
            ["@babel/plugin-transform-runtime"],
        ].filter(Boolean),
        overrides: [
            {
                test: /\.ts$/,
                plugins: [["@babel/plugin-transform-typescript"]],
            },
            {
                test: /\.tsx$/,
                plugins: [["@babel/plugin-transform-typescript", {isTSX: true}]],
            },
        ],
    };
};
