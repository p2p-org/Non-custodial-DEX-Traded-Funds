/* eslint-disable  react-hooks/rules-of-hooks */

const path = require('path');

const {
  useBabelRc,
  override,
  addWebpackModuleRule,
  addWebpackPlugin,
  removeModuleScopePlugin,
  babelInclude,
} = require('customize-cra');

const SpritePlugin = require('svg-sprite-loader/plugin');

module.exports = override(
  useBabelRc(),
  addWebpackPlugin(new SpritePlugin()),
  removeModuleScopePlugin(),
  babelInclude([
    path.resolve(__dirname, 'src'),
    path.resolve(__dirname, '../js/lib'),
  ]),
  addWebpackModuleRule(
    {
      test: /\.tsx?$/,
      exclude: /node_modules/,
      include: [
        path.resolve(__dirname, 'src'),
        path.resolve(__dirname, '../js/lib'),
      ],
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
