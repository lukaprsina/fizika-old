import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";

async function initMarkdownEditor() {
    const easymde = await import("easymde");

    new easymde.default({
        element: document.getElementById("easymde") ?? undefined,
        autosave: {
            enabled: true,
            uniqueId: "simplemde_autosave",
            delay: 15 * 1000
        },
        sideBySideFullscreen: false
    })
}

const [editorInitialized, setEditorInitialized] = createSignal(false);

const MarkdownEditor: Component = () => {
    onMount(async () => {
        if (editorInitialized())
            return;

        await initMarkdownEditor();
        setEditorInitialized(true);
    })

    return <>
        <link rel="stylesheet" href="https://unpkg.com/easymde/dist/easymde.min.css" />
        <textarea id="easymde" >Easy MDE</textarea>
    </>
}

export default MarkdownEditor;