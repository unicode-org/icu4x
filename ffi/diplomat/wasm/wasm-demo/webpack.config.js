export default {
  entry: './src/app.ts',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ]
  },
  resolve: {
    extensions: ['.ts', '.js'],
    fallback: {
      "fs": false,
    },
  },
  mode: "production",
  // mode: "development",
  output: {
    filename: 'bundle.js',
    path: new URL('dist', import.meta.url).pathname,
  },
  experiments: {
    topLevelAwait: true,
  }
};
