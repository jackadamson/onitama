/* eslint-disable */
const path = require('path');
const { override } = require('customize-cra');

const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const wasmExtensionRegExp = /\.wasm$/;

module.exports = override(
  // make the file loader ignore wasm files
  (config) => {
    config.resolve.extensions.push('.wasm');
    config.module.rules.find((rule) => {
      return (rule.oneOf || []).find((item) => {
        if (item.loader && item.loader.indexOf('file-loader') >= 0) {
          item.exclude.push(wasmExtensionRegExp); //exclude wasm
          return true; //ignore remaining rules
        }
      });
    });

    return config;
  },

  //hook up our helloHelper wasm module
  (config) => {
    config.plugins = (config.plugins || []).concat([
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, './onitamalib'),
        outDir: path.resolve(__dirname, './src/onitamalib'),
        extraArgs: '-- --features=web  --features=agent',
      }),
    ]);

    return config;
  },
  (config) => {
    config.resolve.alias.onitamalib = path.resolve(__dirname, 'src/onitamalib/index.js')
    config.module.rules.forEach((rule) => {
      (rule.oneOf || []).forEach((oneOf) => {
        if (oneOf.type === 'asset/resource') {
          oneOf.exclude.push(wasmExtensionRegExp);
        }
      })
    })
    config.experiments = {
      syncWebAssembly: true,
      topLevelAwait: true,
    }
    return config;
  },
  (config) => {
    (config.optimization.minimizer || []).forEach((plugin) => {
      if (plugin.constructor.name === 'TerserPlugin') {
        plugin.options.minimizer.options.keep_fnames = true;
        plugin.options.minimizer.options.keep_classnames = true;
      }
    });
    return config;
  },
);
