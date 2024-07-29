<script lang="ts">
    import { greet } from "$wasm";

    let name = "";

    function onClick() {
        greet(name);
    }
</script>

<h1>Hello, world!</h1>

<form on:submit|preventDefault={onClick}>
    <p><input bind:value={name} /></p>
    <p><button type="submit">Say hello!</button></p>
</form>
