/* eslint-disable  react-hooks/rules-of-hooks */
const {
  useBabelRc,
  override,
  addWebpackModuleRule,
  addWebpackPlugin,
} = require('customize-cra');

const SpritePlugin = require('svg-sprite-loader/plugin');

module.exports = override(
  useBabelRc(),
  addWebpackPlugin(new SpritePlugin()),
  addWebpackModuleRule(
    {
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
    },
    {
      test: /\.svg$/,
      use: [
        {
          loader: 'svg-sprite-loader',
        },
      ],
    },
  ),
);
