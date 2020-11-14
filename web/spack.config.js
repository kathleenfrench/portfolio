const { config } = require('@swc/core/spack')
const path = require('path');

const _distDir = path.resolve(__dirname, '../static/assets/js');
const _rootDir = path.resolve(__dirname, 'src');

module.exports = config({
    entry: {
        'web': `${_rootDir}/main.js`
    },
    output: {
        name: 'bundle.js',
        path: `${_distDir}`
    },
    module: {
      rules: [
        {
          test: /\.js$/
        }
      ]
    },
});