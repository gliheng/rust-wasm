module.exports = {
  baseUrl: './',
  configureWebpack: {
    module: {
      rules: [
        {
          test: /\.rs$/,
          use: {
            loader: 'rust-wasm-loader',
            options: {
              path: 'build/',
            }
          }
        },
      ]
    }
  },
};