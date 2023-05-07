const path = require('path');

module.exports = {
  mode: 'production',
  entry: './dist/server/entry-server.js',
  output: {
    path: path.resolve(__dirname, 'build'),
    filename: 'bundle.js',
    library: 'MyLibrary', // Name of the library
    libraryTarget: 'umd', // Universal module definition
    globalObject: 'this', // Ensure the library works in different environments
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
              ['@babel/preset-env', { modules: 'commonjs' }],
              '@babel/preset-react',
            ],
          },
        },
      },
    ],
  },
  resolve: {
    extensions: ['.js'],
  },
  target: 'node',
  externalsPresets: { node: true }, // Exclude Node.js built-in modules
};
