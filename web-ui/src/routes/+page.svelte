<script lang="ts">
    import { generate_bms } from "$wasm";

    let bars = 16;
    let bpm = 150;
    let jackTolerance = 0;
    let chordDensity = [0, 0, 0, 0, 200];
    let scatter = 0;
    let scatterDecayRate = 0.5;
    let seedString = "";

    const chordLabels = ["1分", "2分", "4分", "8分", "16分"];

    function getSeed() {
        if (seedString === "") {
            return BigInt(Date.now());
        } else {
            return BigInt(seedString);
        }
    }

    function downloadURI(uri: string, name: string) {
        const link = document.createElement("a");
        link.download = name;
        link.href = uri;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }

    function dataURI(content: string, mime = "application/octet-stream") {
        return `data:${mime};base64,${btoa(content)}`;
    }

    function onClick() {
        const chordDensityArray = BigUint64Array.from(chordDensity.map(BigInt));
        const seed = getSeed();
        const resultBms = generate_bms(
            bars,
            bpm,
            jackTolerance,
            chordDensityArray,
            scatter,
            scatterDecayRate,
            seed,
        );

        if (resultBms === undefined) {
            alert("BMS の生成に失敗しました。");
            return;
        }

        downloadURI(dataURI(resultBms), "result.bms");
    }
</script>

<h1>BMS自動生成くん</h1>

<form on:submit|preventDefault={onClick}>
    <label>
        <p>小節数</p>
        <input type="text" bind:value={bars} />
    </label>
    <label>
        <p>BPM</p>
        <input type="text" bind:value={bpm} />
    </label>
    <div class="density-grid">
        {#each chordLabels as label}
            <div>{label}</div>
        {/each}
        {#each chordDensity as density}
            <div><input type="test" bind:value={density} /></div>
        {/each}
    </div>
    <label>
        <p>縦連許容度</p>
        <input type="text" bind:value={jackTolerance} />
    </label>
    <label>
        <p>散らばり度</p>
        <input type="text" bind:value={scatter} />
        <p><input type="range" min="-10" max="10" bind:value={scatter}></p>
    </label>
    <label>
        <p>シード</p>
        <input type="text" bind:value={seedString} />
    </label>
    <p>
        <button type="submit">生成</button>
    </p>
</form>

<style>
    label {
        display: block;
        margin: 20px 0;
    }

    label p {
        margin: 5px 0;
        color: rgba(0, 0, 0, 0.8);
    }

    input {
        padding: 5px;
    }

    button {
        box-sizing: border-box;
    }

    .density-grid {
        display: grid;
        grid-template-columns: repeat(5, 100px);
        gap: 5px;
    }

    .density-grid input {
        box-sizing: border-box;
        width: 100%;
    }
</style>
