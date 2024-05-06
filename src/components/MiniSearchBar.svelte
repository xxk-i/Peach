<script lang="ts">
	import { window } from "@tauri-apps/api";
	import type { Writable } from "svelte/store";

    export let filter: Writable<string>;
    
    let showInput = false;
    let input: HTMLInputElement;

    function tryRemoveFocus(event: KeyboardEvent) {
        if (event.key === 'Enter' || event.key === 'Escape') {
            input.blur()
        }
    }

    function tryUnshowInput() {
        if ($filter.length == 0) {
            showInput = false;
        }
    }
</script>

{#if !showInput}
    <button class="rounded-full bg-peach-400 leading-[10px] min-h-7 min-w-7" on:click={() => showInput = true}>
        <span class="material-symbols-outlined">
            search
        </span> 
    </button>
{:else}
    <input bind:this={input} bind:value={$filter} on:focusout={tryUnshowInput} on:keydown={tryRemoveFocus} autofocus/>
{/if}

<style>
    button {
        text-align: right;
        padding-right: 5%;
        min-width: 1.75rem;
        transition: min-width 0.25s;
    }

    button:hover {
        text-align: right;
        padding-right: 5%;
        min-width: 4rem;
        transition: min-width 0.25s;
    }
</style>
<!-- <input bind:value={$filter}/> -->