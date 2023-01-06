import type { Component } from "solid-js";
import { Show } from "solid-js";
import { For, createSignal } from "solid-js"
import { createDropzone } from "@solid-primitives/upload"
import { A, useParams } from "solid-start";
import type { Page } from "@prisma/client";

type FileManagerType = {
    page?: Page | null
}

const FileManager: Component<FileManagerType> = (props) => {
    const { setRef: dropzoneRef, files: droppedFiles } = createDropzone({
    })

    const params = useParams();

    const [files, setFiles] = createSignal<string[]>(["A", "B", "C"]);

    return (
        <div
            class="w-screen h-screen fixed top-0 left-0 flex justify-center items-center"
        >
            <div
                class="fixed bg-neutral-700 z-50 w-1/2 outline-white outline outline-2 rounded-xl"
                ref={dropzoneRef}
            >
                <div class="bg-neutral-800 p-3">
                    <Show when={params.topic}>
                        <A href="test">{params.topic}</A>
                    </Show>
                    <Show when={params.topic}>
                        <span>{" / "}</span>
                        <A href="test">{props.page?.title}</A>
                    </Show>
                </div>
                <div class="p-3">
                    <For each={files()}>{(file) =>
                        <div>{file}</div>
                    }
                    </For>
                </div>
            </div>
        </div >
    )
}

export default FileManager