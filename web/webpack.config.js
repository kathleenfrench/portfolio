const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const staticDir = path.resolve(__dirname, '../static');
const dist = path.resolve(__dirname, '../dist');

// outputs (all served by the rust server):
//   dist/assets/js/      hashed js bundles + wasm (cached forever, see src/cache.rs)
//   dist/                everything in static/ (it mirrors this layout)
//   dist/templates/      handlebars templates; index.html gets the bundle <script> injected
module.exports = {
  mode: 'production',
  entry: './src/main.js',
  output: {
    path: path.join(dist, 'assets/js'),
    publicPath: '/assets/js/',
    filename: '[name].[contenthash:8].js',
    chunkFilename: '[name].[contenthash:8].js',
    webassemblyModuleFilename: '[hash].module.wasm',
    clean: true,
  },
  experiments: {
    asyncWebAssembly: true,
  },
  performance: {
    // the wasm terminal is ~500KB; that's expected, don't warn about it
    hints: false,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(staticDir, 'templates/index.html'),
      filename: path.join(dist, 'templates/index.html'),
      inject: 'body',
      minify: false,
    }),
    new CopyPlugin({
      patterns: [
        {
          from: staticDir,
          to: dist,
          // index.html is emitted by HtmlWebpackPlugin above instead
          filter: (file) =>
            file !== path.join(staticDir, 'templates/index.html') && !file.endsWith('.DS_Store'),
        },
        {
          from: require.resolve('xterm/css/xterm.css'),
          to: path.join(dist, 'assets/css/xterm.css'),
        },
      ],
    }),
  ],
};
