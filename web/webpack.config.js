const path = require('path');
const _staticDir = path.resolve(__dirname, '../static');
const _crateDir = path.resolve(__dirname, '../pterm');
const _crateOut = path.resolve(__dirname, 'pkg')
const _rootDir = path.resolve(__dirname, 'src');
const _dist = path.resolve(__dirname, "../dist");

const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  // see: https://github.com/webpack/webpack/issues/6615
  entry : `${_rootDir}/main.js`,
  output : {
    path : `${_dist}/assets/js`,
    filename : "bundle.js",
  },
  mode : "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CleanWebpackPlugin(),
    new CopyPlugin([
      { from: _staticDir, to: `${_dist}`, ignore: ['*.css'] },
      { from: `${_staticDir}/css`, to: `${_dist}/assets/css` },
    ]),
    new WasmPackPlugin({ 
      crateDirectory: _crateDir,
      outDir: _crateOut,
      extraArgs: "--no-typescript",
      forceMode: 'release',
    }),
  ]
};