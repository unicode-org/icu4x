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
		'./src/js/index.mjs',
		'./src/scss/styles.scss',
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