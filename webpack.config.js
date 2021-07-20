const path = require("path")

module.exports = {
  entry: {
    bundle: "./src/index.js"
  },

  // Reload if any changes
  watch: true,

  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "out")
  },

  mode: "production",
  devtool: "source-map",

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: [
          /node_modules/
        ],
        use: [
          { loader: "babel-loader" }
        ]
      }
    ]
  }
}