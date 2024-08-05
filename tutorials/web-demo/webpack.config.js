import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
	entry: {
	  index: [
		'./src/index.mjs',
	  ],
	  rendering: [
		'./src/runtime.mjs'
	  ]
	},
	module: {
	  rules: [
		
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