import type { Component } from "solid-js";
import { createSignal } from "solid-js";
import { Body } from "solid-start";
import { onMount } from "solid-js";
import loader from '@monaco-editor/loader';
import { createDropzone } from "@solid-primitives/upload";
import type monaco from 'monaco-editor'

/* const selection = editor().getPosition();

    editor().executeEdits("file upload", [{
        range: selection?.collapseToStart(),
        text: "XXX",
        forceMoveMarkers: true
    }]) */

const MonacoEditor: Component = () => {
    const [editor, setEditor] = createSignal<monaco.editor.IStandaloneCodeEditor>()

    const { setRef: dropzoneRef } = createDropzone({
        onDrop: async files => {
            const formData = new FormData();
            files.forEach(file => formData.append("files", file.file));

            await fetch("api/upload", {
                method: "POST",
                headers: new Headers({
                    'content-type': 'multipart/form-data'
                }),
                body: formData,
            })
        }
    })

    onMount(() => {
        loader.init().then(monaco => {
            const component = document.querySelector("#editor");
            if (!component)
                return;

            const new_editor = monaco.editor.create(component as HTMLElement, {
                value: '# editor',
                language: 'markdown',
                dragAndDrop: true,
            });

            setEditor(new_editor);
        });
    })

    return <div
        id="editor"
        ref={dropzoneRef}
        class="w-screen h-screen"
    />
}

export default MonacoEditor;