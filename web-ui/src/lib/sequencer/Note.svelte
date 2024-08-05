<script lang="ts">
    export let height = 15;
    export let width = 50;
    export let x: number;
    export let y: number;
    export let length: number;
</script>

<div
    class="note"
    style:height="{height}px"
    style:width="{width * length}px"
    style:left="{x * width}px"
    style:top="{y * height}px"
    on:click
/>

<style>
    .note {
        position: absolute;
        box-sizing: border-box;
        background-color: #ff9922;
        border: 1px solid #aa5500;
    }
</style>
