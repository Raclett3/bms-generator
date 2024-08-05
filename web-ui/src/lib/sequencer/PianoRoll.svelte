<script lang="ts">
    import Note from "./Note.svelte";
    import PianoRollRow from "./PianoRollRow.svelte";

    type Drag = {
        originX: number;
        originY: number;
        currentX: number;
        currentY: number;
    };

    type NoteParams = {
        x: number;
        y: number;
        length: number;
    };

    const keyColors = [
        false,
        true,
        false,
        true,
        false,
        true,
        false,
        false,
        true,
        false,
        true,
        false,
    ];
    const height = 15;
    const C4 = 440 * Math.pow(2.0, -9 / 12);

    export let rows: number;
    export let columns: number;
    export let width: number;

    let ongoingDrag: Drag | null = null;
    let currentNote: NoteParams | null = null;
    let notes: Record<number, NoteParams> = {};

    const audioCtx = new AudioContext();
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = "sawtooth";
    osc.connect(gain);
    gain.gain.value = 0;
    gain.connect(audioCtx.destination);
    osc.start();

    function dragToNote(drag: Drag | null): NoteParams | null {
        if (drag === null) {
            return null;
        }

        let length: number;
        let x: number;
        const y = drag.currentY;

        if (drag.originX <= drag.currentX) {
            length = drag.currentX - drag.originX + 1;
            x = drag.originX;
        } else {
            length = drag.originX - drag.currentX;
            x = drag.currentX;
        }

        return { x, length, y };
    }

    $: currentNote = dragToNote(ongoingDrag);

    const addNote = (() => {
        let generatedNotes = 0;

        return (note: NoteParams) => {
            notes[generatedNotes] = note;
            generatedNotes++;
        };
    })();

    function removeNote(key: number) {
        return () => {
            delete notes[key];
            notes = notes;
        };
    }

    function noteOn(y: number) {
        osc.frequency.value = C4 * Math.pow(2, (rows * 12 - y - 1) / 12);
        gain.gain.value = 0.2;
    }

    function noteOff() {
        gain.gain.value = 0;
    }

    function mousedown(y: number, row: number, column: number) {
        return (x: number) => {
            const offsetX = x + column * 16;
            const offsetY = y + row * 12;
            noteOn(offsetY);
            ongoingDrag = {
                originX: offsetX,
                originY: offsetY,
                currentX: offsetX,
                currentY: offsetY,
            };
        };
    }

    function mousemove(event: MouseEvent) {
        if (ongoingDrag === null || !(event.currentTarget instanceof HTMLElement)) {
            return;
        }

        const targetRect = event.currentTarget.getBoundingClientRect();
        const offsetX = event.clientX - targetRect.left;
        const offsetY = event.clientY - targetRect.top;
        const x = Math.min(Math.floor(offsetX / width), columns * 16 - 1);
        const y = Math.min(Math.floor(offsetY / height), rows * 12 - 1);
        noteOn(y);
        ongoingDrag.currentX = x;
        ongoingDrag.currentY = y;
    }

    function mouseup() {
        noteOff();
        ongoingDrag = null;
        if (currentNote) {
            addNote(currentNote);
        }
    }
</script>

<svelte:window on:mouseup={mouseup} />

<div class="bar" on:mousemove|preventDefault={mousemove}>
    {#each new Array(rows) as _, row (row)}
        <div class="row">
            {#each new Array(columns) as _, column (column)}
                <div>
                    {#each keyColors as color, noteIdx}
                        <PianoRollRow
                            black={color}
                            length={16}
                            {height}
                            {width}
                            borderTop={noteIdx === 0}
                            onMousedown={mousedown(noteIdx, row, column)}
                        />
                    {/each}
                </div>
            {/each}
        </div>
    {/each}

    {#each Object.entries(notes) as [key, note] (key)}
        <Note {...note} {width} {height} on:click={removeNote(Number(key))} />
    {/each}

    {#if currentNote}
        <Note {...currentNote} {width} {height} />
    {/if}
</div>

<style>
    .bar {
        position: relative;
        width: 100%;
        height: 100%;
        overflow-x: scroll;
    }

    .row {
        display: flex;
        margin: 0;
    }
</style>
