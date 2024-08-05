<script lang="ts">
    import PianoRollCell from "./PianoRollCell.svelte";

    export let black: boolean;
    export let borderTop: boolean;
    export let length: number;
    export let height: number;
    export let width: number;
    export let onMousedown: ((x: number) => unknown) | undefined = undefined;

    function onMousedownHandler(i: number) {
        return () => {
            onMousedown?.(i);
        };
    }
</script>

<div class="row">
    {#each new Array(length) as _, i (i)}
        <div class="cell">
            <PianoRollCell
                {black}
                {height}
                {width}
                borderLeft={i % 4 === 0}
                {borderTop}
                on:mousedown={onMousedownHandler(i)}
            />
        </div>
    {/each}
</div>

<style>
    .row {
        box-sizing: border-box;
        display: flex;
    }

    .cell {
        margin: 0;
        padding: 0;
    }
</style>
