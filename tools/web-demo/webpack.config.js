// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import path from "path";
import { fileURLToPath } from "url";
import MiniCssExtractPlugin from "mini-css-extract-plugin";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
	plugins: [new MiniCssExtractPlugin({
		filename: "[name].css"
	})],
	entry: {
		index: [
			'./src/index.mjs',
			'./src/style.scss',
		]
	},
	module: {
		rules: [
			{
				test: /\.(scss)$/,
				use: [
					MiniCssExtractPlugin.loader,
					"css-loader",
					{
						loader: 'sass-loader'
					}
				]
			},
			{
				test: /\.(wasm)$/,
				generator: {
					filename: '[name].wasm?v=[hash]'
				}
			}
		]
	},
	resolve: {
		extensions: ['.mjs', '.js'],
		fallback: {
			"fs": false,
		},
	},
	// mode: "production",
	mode: "development",
	output: {
		filename: '[name].bundle.js',
		path: path.resolve(__dirname, 'public/dist'),
	},
	devServer: {
		static: '.',
		port: 8080,
		hot: true,
		headers: {
			"Access-Control-Allow-Origin": "*",
			"Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
			"Access-Control-Allow-Headers": "X-Requested-With, content-type, Authorization"
		}
	},
	experiments: {
		topLevelAwait: true,
	},
	optimization: {
		minimize: false
	},
};