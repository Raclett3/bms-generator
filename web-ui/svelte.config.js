import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),
        alias: {
            $wasm: "./wasm/pkg",
        },
        paths: {
            base: process.argv.includes("dev") ? "" : "/bms-generator",
        },
    },
};

export default config;
