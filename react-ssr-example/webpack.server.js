const path = require("path");
const buildDirectory = "../build";

module.exports = [{
  mode: "production",
  entry: "./src/ssr.js",
  target: "web",
  output: {
    path: path.resolve(__dirname, buildDirectory),
    library: "SSR",
    libraryTarget: "var",
    filename: "index.js",
  },
  resolve: {
    extensions: [".js", ".jsx", ".json", ".ts", ".tsx"],
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: "babel-loader",
      },
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: "babel-loader",
      },
      {
        test: /\.svg$/,
        use: [
          {
            loader: 'url-loader',
            options: {
              limit: 8192, // Adjust this limit as needed
              name: 'static/media/[name].[hash:8].[ext]',
            },
          },
        ],
      },
    ],
  },
}, {
  mode: "production",
  target: "web",
  entry: path.resolve(__dirname, "./src/index.jsx"),
  output: {
    path: path.resolve(__dirname, buildDirectory),
    filename: "scripts/bundle.js",
  },
  resolve: {
    extensions: [".js", ".jsx", ".json", ".ts", ".tsx"],
  },
}];
