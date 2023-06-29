const path = require('path');

module.exports = {
  mode: 'production',
  entry: './dist/server/entry-server.js',
  output: {
    path: path.resolve(__dirname, 'dist/server'),
    filename: 'entry-server.wasm',
    library: 'MyLibrary',
    libraryTarget: 'module',
    module: true,
    environment: {
      module: true,
    },
    globalObject: 'this',
  },
  experiments: {
    outputModule: true,
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              ['@babel/preset-env', { modules: false }],
              '@babel/preset-react',
            ],
          },
        },
      },
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
        use: {
          loader: 'wasm-loader',
        },
      },
    ],
  },
  resolve: {
    extensions: ['.js', '.wasm'],
    alias: {
      'react': path.resolve(__dirname, 'node_modules/react/index.js'),
      'react-dom': path.resolve(__dirname, 'node_modules/react-dom/index.js'),
    },
    fallback: {
      util: require.resolve("util/"),
    },
  },
  target: 'web', // Changed from 'node'
  externalsPresets: { node: true },
};
