<script lang="ts">
    export let black = false;
    export let height = 15;
    export let width = 50;
    export let borderLeft = false;
    export let borderTop = false;
</script>

<div
    class="row"
    class:black
    class:border-top={borderTop}
    class:border-left={borderLeft}
    style:width="{width}px"
    style:height="{height}px"
    on:mousedown|preventDefault
/>

<style>
    .row {
        box-sizing: border-box;
        background-color: #ccccff;
        margin: 0;
        border-top: 1px solid rgba(0, 0, 0, 0.5);
    }

    .row.black {
        background-color: #aaaaff;
    }

    .border-top {
        border-top: 1px solid rgba(255, 255, 255, 0.8);
    }

    .border-left {
        border-left: 1px solid rgba(255, 255, 255, 0.8);
    }
</style>
