const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: './src/index.ts',
  mode: 'development',
  devServer: {
    allowedHosts: "all",
    client: {
        webSocketURL: 'ws://0.0.0.0/ws',
    },
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './public/index.html'
    }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  experiments: {
    asyncWebAssembly: true,
  },
}