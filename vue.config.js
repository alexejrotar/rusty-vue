const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  configureWebpack: {
    plugins: [
      new WasmPackPlugin({ crateDirectory: path.resolve(__dirname, ".")})
    ]
  }
}
