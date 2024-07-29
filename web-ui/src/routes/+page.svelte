<script lang="ts">
    import { base } from "$app/paths";
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
            "Auto Generated",
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

<details>
    <summary><h2>つかいかた</h2></summary>
    <!-- prettier-ignore -->
    <div class="usage">
        <p>パラメータを指定して生成ボタンを押すことで、ランダムなBMSの譜面を生成することが出来ます。</p>
        <p><a data-sveltekit-reload href="{base}/bms-generator.zip">こちらのキー音</a>と同一のフォルダに入れてプレイヤーで読み込んでください。</p>
        <h3>譜面密度</h3>
        <p>n分間隔で降ってくるノーツの数を指定することが出来ます。</p>
        <p>例えば、100%を指定した場合1つ押し、200%を指定した場合は2つ押し、150%を指定した場合は1つ押しと2つ押しが半分ずつ生成されます。</p>
        <h3>縦連許容度</h3>
        <p>許容される縦連の長さを指定できます。</p>
        <p>xを指定した場合、x+1個までの縦連が生成されるようになります。</p>
        <p>小数を指定することもでき、例えば1.2を指定した場合は3個までの縦連が生成されますが、2を指定した場合に比べ出現率が下がります。</p>
        <h3>散らばり度</h3>
        <p>高い値を設定するほど、譜面の偏りを減らすことができます。負の値を設定することで、偏りを増やすことができます。</p>
        <p><b>極端に高い値(100など)を設定した場合、特定のパターンを繰り返す譜面が生成される場合があります。</b></p>
    </div>
</details>

<form on:submit|preventDefault={onClick}>
    <div class="form-group">
        <h2>基本情報</h2>
        <div class="form-flex">
            <label>
                <p>小節数</p>
                <input type="text" bind:value={bars} />
            </label>
            <label>
                <p>BPM</p>
                <input type="text" bind:value={bpm} />
            </label>
        </div>
    </div>
    <div class="form-group">
        <h2>譜面密度</h2>
        <div class="form-flex">
            {#each chordDensity as density, i}
                <label>
                    <p>{chordLabels[i]}</p>
                    <input type="test" bind:value={density} />
                    <div class="unit">%</div>
                </label>
            {/each}
        </div>
    </div>
    <div class="form-group">
        <details>
            <summary><h2>詳細設定</h2></summary>
            <label>
                <p>縦連許容度</p>
                <input type="text" bind:value={jackTolerance} />
            </label>
            <div>
                <label>
                    <p>散らばり度</p>
                    <input type="text" bind:value={scatter} />
                </label>
                <p><input type="range" min="-10" max="10" bind:value={scatter} /></p>
            </div>
            <label>
                <p>シード</p>
                <input type="text" bind:value={seedString} />
            </label>
        </details>
    </div>
    <p>
        <button type="submit">生成</button>
    </p>
</form>

<style>
    .usage {
        padding: 3px 7px;
        border: 1px solid rgba(0, 0, 0, 0.2);
        font-size: 90%;
    }

    form {
        box-sizing: border-box;
        width: 100%;
        max-width: 720px;
        padding: 20px;
    }

    label {
        display: block;
        margin: 20px 0;
    }

    label > p {
        margin: 5px 0;
        color: rgba(0, 0, 0, 0.8);
    }

    input {
        box-sizing: border-box;
        padding: 5px;
        width: 100%;
    }

    .unit {
        width: 100%;
        text-align: right;
    }

    button {
        box-sizing: border-box;
        width: 100%;
        padding: 5px;
        appearance: none;
        font: inherit;
        border: none;
        border-radius: 3px;
        color: #ffffff;
        background-color: #0088ff;
    }

    .form-group {
        box-sizing: border-box;
        width: 100%;
        border: 1px solid rgba(0, 0, 0, 0.3);
        border-radius: 3px;
        margin: 20px 0;
        padding: 5px 10px;
    }

    h2 {
        margin: 5px 0;
    }

    summary::marker {
        font-size: 150%;
    }

    summary h2 {
        display: inline;
    }

    .form-flex {
        display: flex;
        grid-template-columns: repeat(5, 100px);
        gap: 5px;
    }

    .form-flex > * {
        flex-grow: 1;
    }
</style>
