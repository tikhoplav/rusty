const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: './src/index.ts',
  mode: 'development',
  devServer: {
    allowedHosts: "all",
    client: {
      webSocketURL: 'ws://0.0.0.0/ws',
    },
    watchFiles: [
      "./target/wasm32-unknown-unknown/release/rusty.wasm"
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './public/index.html'
    }),
    new CopyPlugin({
      patterns: [
        "./target/wasm32-unknown-unknown/release/rusty.wasm"
      ],
    }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    alias: {
      utils: path.resolve(__dirname, "src/utils"),
    }
  },
}