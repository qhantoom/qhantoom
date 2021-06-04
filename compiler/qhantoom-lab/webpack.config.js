module.exports = (_, args) => {
  const path = require('path');
  const HtmlWebpackPlugin = require('html-webpack-plugin');
  const webpack = require('webpack');
  const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
  const prod = (args.mode === 'production'); //package.json scripts -> build

  return {
    entry: './src/runtime/index.js',
    output: {
      path: path.resolve(__dirname, 'out'),
      // filename: 'qhantoom-lab.js',
      filename: prod ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    plugins: [
      new HtmlWebpackPlugin({
        title: "qhantoom lab",
        template: 'index.html',
        inject: true,
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "."),
      }),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder'],
      }),
    ],
    mode: 'development',
  };
};
