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

const path = require('path');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

/**
 * @type {function(webpack.Configuration, webpack.Configuration): webpack.Configuration}
 */
const webpackMerge = require('webpack-merge');

/**
 *  @type {webpack.Configuration}
 */
let config = {
    target: 'web',
    entry: './src/index.tsx',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
    },
    plugins: [
        new HtmlWebpackPlugin({template: './src/index.html'}),
        new ForkTsCheckerWebpackPlugin(),
    ],
    resolve: {
        modules: ['node_modules'],
        extensions: ['.ts', '.tsx', '.js', '.jsx', '.json'],
    },
    module: {
        rules: [
            {test: /\.css$/, use: [{loader: 'style-loader'}, {loader: 'css-loader'}]},
            {test: /\.(ts|tsx)$/, use: [{loader: 'babel-loader'}, {loader: 'eslint-loader'}], exclude: /node_modules/},
            {test: /\.(png|svg|jpg|git|ttf|woff|woff2|eot)$/, use: [{loader: 'file-loader'}]},
        ],
    },
};

// noinspection JSUndefinedPropertyAssignment
/**
 * @type {WebpackDevServer.Configuration}
 */
config.devServer = {
    host: '0.0.0.0',
    hot: true,
    overlay: {
        warnings: true,
        errors: true,
    },
};

if (process.env.NODE_ENV === 'production') {
    config = webpackMerge(config, {
        mode: 'production',
        devtool: 'source-map',
    });
} else {
    config = webpackMerge(config, {
        mode: 'development',
        devtool: 'inline-source-map',
    });
}

if (process.env.BUNDLE_ANALYZER) {
    const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;
    config = webpackMerge(config, {
        plugins: [new BundleAnalyzerPlugin()],
    });
}

module.exports = config;
