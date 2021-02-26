/* eslint-disable  react-hooks/rules-of-hooks */
const { useBabelRc, override, addWebpackModuleRule } = require('customize-cra');

module.exports = override(
  useBabelRc(),
  addWebpackModuleRule({
    test: /\.tsx?$/,
    exclude: /node_modules/,
    use: [
      { loader: 'babel-loader', options: { cacheDirectory: true } },
      {
        loader: '@linaria/webpack-loader',
        options: {
          cacheDirectory: 'src/.linaria_cache',
          sourceMap: process.env.NODE_ENV !== 'production',
        },
      },
    ],
  }),
);
