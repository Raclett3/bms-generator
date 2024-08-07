<script lang="ts">
    import PianoRoll, { type MusicalNote } from "$lib/sequencer/PianoRoll.svelte";

    const A4 = 440;

    let notes: MusicalNote[] = [];

    const bpm = 150;

    function play() {
        const audioCtx = new AudioContext();

        const gain = audioCtx.createGain();
        gain.gain.value = 0.5;
        gain.connect(audioCtx.destination);

        for (const note of notes) {
            const startFrom = (240 / bpm / 16) * note.position;
            const endsAt = startFrom + (240 / bpm / 16) * note.length;

            const osc = audioCtx.createOscillator();
            osc.frequency.value = A4 * Math.pow(2, note.note / 12);
            osc.type = "sawtooth";
            osc.connect(gain);
            osc.start(audioCtx.currentTime + startFrom);
            osc.stop(endsAt);
        }
    }
</script>

<PianoRoll rows={2} columns={2} width={40} bind:notes />

<button type="button" on:click={play}>Play</button>
