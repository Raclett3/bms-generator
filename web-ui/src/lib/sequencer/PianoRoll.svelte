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

    export let rows: number;
    export let columns: number;
    export let width: number;

    let ongoingDrag: Drag | null = null;
    let currentNote: NoteParams | null = null;
    let notes: Record<number, NoteParams> = {};

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

    function mousedown(y: number, row: number, column: number) {
        return (x: number) => {
            ongoingDrag = {
                originX: x + column * 16,
                originY: y + row * 12,
                currentX: x + column * 16,
                currentY: y + row * 12,
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
        const x = Math.floor(offsetX / width);
        const y = Math.floor(offsetY / height);
        ongoingDrag.currentX = Math.floor(offsetX / width);
        ongoingDrag.currentY = Math.floor(offsetY / height);
    }

    function mouseup() {
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
