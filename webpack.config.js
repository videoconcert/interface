const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WebpackNotifierPlugin = require('webpack-notifier');

const distPath = path.resolve(__dirname, "dist");
const appConfig = (env, argv) => {
  return {
    entry: './app/bootstrap.js',
    output: {
      path: distPath,
      filename: "[name].js",
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "app"),
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};

const workerConfig = (env, argv) => {
  return {
    entry: './webworker/worker.js',
    target: 'webworker',
    output: {
      path: distPath,
      publicPath: 'http://localhost:8000/',
      filename: "worker.bundle.js",
      webassemblyModuleFilename: "worker.wasm"
    },
    plugins: [
      new WebpackNotifierPlugin({title: 'worker.js'}),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "webworker"),
        extraArgs: "--no-typescript",
      }),
    ],
    watch: argv.mode !== 'production'
  }
};

module.exports = (env, argv) => {
  return [appConfig(env, argv), workerConfig(env, argv)]
}