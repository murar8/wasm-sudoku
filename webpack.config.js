const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: "production",

    entry: "./js",

    output: {
        filename: "index.js",
    },

    devtool: "source-map",

    devServer: {
        hot: false,
        watchFiles: ["static/**/*"],
        static: { directory: "./dist" },
    },

    plugins: [new CopyPlugin({ patterns: ["./static"] }), new WasmPackPlugin({ crateDirectory: __dirname })],

    experiments: { asyncWebAssembly: true, topLevelAwait: true },

    ignoreWarnings: [
        // See https://github.com/rust-random/getrandom/issues/224
        (warning) => warning.message === "Critical dependency: the request of a dependency is an expression",
    ],
};
