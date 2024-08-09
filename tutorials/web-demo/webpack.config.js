import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
	entry: {
	  index: [
		'./src/js/index.mjs',
		'./src/scss/styles.scss',
	  ],
	  rendering: [
		'./src/js/runtime.mjs'
	  ]
	},
	module: {
	  rules: [
		{
			test: /\.(scss)$/,
			use: [
				{
				loader: 'style-loader',
				},
				{
				loader: 'css-loader'
				},
				{
				loader: 'postcss-loader',
				options: {
					postcssOptions: {
					plugins: () => [
						require('autoprefixer')
					]
					}
				}
				},
				{
				loader: 'sass-loader'
				}
			]
		}
	  ]
	},
	resolve: {
	  extensions: ['.mjs'],
	  fallback: {
		"fs": false,
	  },
	},
	// mode: "production",
	mode: "development",
	output: {
	  filename: '[name].bundle.js',
	  path: path.resolve(__dirname, 'dist'),
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