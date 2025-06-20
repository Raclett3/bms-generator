<script lang="ts">
    export let chordDensity: number[];

    const chordLabels = ["1分", "2分", "4分", "8分", "16分"];
</script>

<h2>譜面密度</h2>
<div class="form-flex">
    {#each chordDensity as density, i}
        <label>
            <p>{chordLabels[i]}</p>
            <input type="text" bind:value={density} />
            <div class="unit">%</div>
        </label>
    {/each}
</div>

<style>
    label {
        display: block;
    }

    label > p {
        margin: 5px 0;
        color: rgba(0, 0, 0, 0.8);
    }

    input[type="text"] {
        box-sizing: border-box;
        padding: 5px;
        width: 100%;
    }

    .unit {
        width: 100%;
        text-align: right;
    }

    h2 {
        margin: 5px 0;
    }

    .form-flex {
        display: flex;
        flex-wrap: wrap;
        gap: 5px;
        padding: 20px 0;
    }

    .form-flex > * {
        flex: 1 0 0px;
        min-width: 80px;
    }
</style>
