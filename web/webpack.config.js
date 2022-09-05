const path = require('path');
const _staticDir = path.resolve(__dirname, '../static');
const _crateDir = path.resolve(__dirname, 'pterm');
const _crateOut = path.resolve(__dirname, 'pkg')
const _rootDir = path.resolve(__dirname, 'src');
const _dist = path.resolve(__dirname, "../dist");

const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const PreloadWebpackPlugin = require("@vue/preload-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  // see: https://github.com/webpack/webpack/issues/6615
  entry : `${_rootDir}/main.js`,
  output : {
    path : `${_dist}/assets/js`,
    filename : "bundle.js",
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin(),
    new PreloadWebpackPlugin({
      rel: "preconnect",
      fileWhitelist: [/.wasm$/],
    }),
    new CopyPlugin([
      { from: _staticDir, to: `${_dist}`, ignore: ['*.css', '*.js'] },
      { from: `${_staticDir}/css`, to: `${_dist}/assets/css` },
      { from: `${_staticDir}/js`, to: `${_dist}/assets/js` }
    ]),
    new WasmPackPlugin({ 
      crateDirectory: _crateDir,
      outDir: _crateOut,
      forceMode: 'production',
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};