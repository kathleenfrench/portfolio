const path = require('path');
const _distDir = path.resolve(__dirname, '../static/assets/js');
const _crateDir = path.resolve(__dirname, '../pterm');
const _crateOut = path.resolve(__dirname, 'pkg')
const _rootDir = path.resolve(__dirname, 'src');

const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  // see: https://github.com/webpack/webpack/issues/6615
  entry : `${_rootDir}/main.js`,
  output : {
    path : `${_distDir}`,
    filename : "bundle.js",
  },
  mode : "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new WasmPackPlugin({ 
      crateDirectory: _crateDir,
      outDir: _crateOut,
      extraArgs: "--no-typescript",
    }),
  ]
};