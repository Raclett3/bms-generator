<script context="module" lang="ts">
    export type MusicalNote = {
        position: number;
        note: number;
        length: number;
    };
</script>

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
    const A4 = 440;

    export let rows: number;
    export let columns: number;
    export let width: number;
    export let notes: MusicalNote[] = [];

    let ongoingDrag: Drag | null = null;
    let currentNote: NoteParams | null = null;
    let notesMap: Record<number, NoteParams> = {};

    const audioCtx = new AudioContext();
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = "sawtooth";
    osc.connect(gain);
    gain.gain.value = 0;
    gain.connect(audioCtx.destination);
    osc.start();

    function yToNote(y: number): number {
        const offsetFromC4 = rows * 12 - y - 1;
        return offsetFromC4 - 9;
    }

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
    $: notes = Object.values(notesMap).map((note) => ({
        position: note.x,
        note: yToNote(note.y),
        length: note.length,
    }));

    const addNote = (() => {
        let generatedNotes = 0;

        return (note: NoteParams) => {
            notesMap[generatedNotes] = note;
            generatedNotes++;
        };
    })();

    function removeNote(key: number) {
        return () => {
            delete notesMap[key];
            notesMap = notesMap;
        };
    }

    function noteOn(note: number) {
        osc.frequency.value = A4 * Math.pow(2, note / 12);
        gain.gain.value = 0.2;
    }

    function noteOff() {
        gain.gain.value = 0;
    }

    function mousedown(y: number, row: number, column: number) {
        return (x: number) => {
            const offsetX = x + column * 16;
            const offsetY = y + row * 12;
            noteOn(yToNote(offsetY));
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
        noteOn(yToNote(y));
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

    {#each Object.entries(notesMap) as [key, note] (key)}
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
