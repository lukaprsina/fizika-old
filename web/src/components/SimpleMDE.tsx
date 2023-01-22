import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";

async function initSimpleMDE() {
    const simplemde = await import("simplemde");
    console.log("Test")
    /* new simplemde.default({
        element: document.getElementById("simplemde") ?? undefined,
        autosave: {
            enabled: true,
            uniqueId: "simplemde_autosave",
            delay: 15 * 1000
        }
    }) */
}

const [editorInitialized, setEditorInitialized] = createSignal(false);

const SimpleMDE: Component = () => {
    onMount(async () => {
        if (editorInitialized())
            return;

        await initSimpleMDE();
        setEditorInitialized(true);
    })

    return <textarea id="simplemde" >Simple MDE</textarea>
}

export default SimpleMDE;