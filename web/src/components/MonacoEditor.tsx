import type { Component } from "solid-js";
import { onMount } from "solid-js";
import loader from '@monaco-editor/loader';

const MonacoEditor: Component = () => {
    onMount(() => {
        loader.init().then(monaco => {
            const component = document.querySelector("#editor");
            if (!component)
                return;

            monaco.editor.create(component as HTMLElement, {
                value: '# editor',
                language: 'markdown',
            });
        });
    })

    return <div
        id="editor"
        class="w-screen h-screen bg-gray-400"
    />
}

export default MonacoEditor;