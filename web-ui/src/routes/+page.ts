import type { PageLoad } from "./$types";
import init from "$wasm";

export const load = (async () => {
    await init();
}) satisfies PageLoad;
