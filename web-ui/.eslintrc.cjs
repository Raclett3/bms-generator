module.exports = {
    root: true,
    parser: "@typescript-eslint/parser",
    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended", "plugin:svelte/recommended", "prettier"],
    plugins: ["@typescript-eslint"],
    overrides: [
        {
            files: ["*.svelte"],
            processor: "svelte-eslint-parser",
            parserOptions: { parser: "@typescript-eslint/parser" }
        }
    ],
    parserOptions: {
        sourceType: "module",
        ecmaVersion: 2020,
        extraFileExtensions: [".svelte"],
    },
    env: {
        browser: true,
        es2017: true,
        node: true,
    },
    rules: {
        indent: ["error", 4],
        "linebreak-style": ["error", "unix"],
        quotes: ["error", "double"],
        semi: ["error", "always"],
        "comma-dangle": ["error", "always-multiline"],
    },
};
