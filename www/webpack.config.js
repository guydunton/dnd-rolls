const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/bootstrap.js',
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
    ],
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  experiments: { syncWebAssembly: true },
  mode: 'development',
  plugins: [
    new CopyWebpackPlugin({
      patterns: ['src/index.html'],
    }),
  ],
};
