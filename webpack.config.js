const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    entry: './js/index.js',
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'dist'),
        clean: true,
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', "css-loader", "postcss-loader"],
            }
        ],
    },
    experiments: {
        syncWebAssembly: true,
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '.'),
            extraArgs: '--no-typescript',
            outDir: path.resolve(__dirname, "pkg"),
        }),
        new HtmlWebpackPlugin({
            title: "Playground",
            meta: {
                viewport: "width=device-width, initial-scale=1.0",
                favicon: "./favicon.ico"
            }
        }),
        new CopyPlugin({
            patterns: [
                { from: "static" },
                { from: "pkg" },
            ],
        }),

    ],
};