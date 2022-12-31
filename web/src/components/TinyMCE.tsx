import { createContextProvider } from "@solid-primitives/context";
import type { Component } from "solid-js";
import { createEffect, createSignal, onCleanup } from "solid-js";
import type { Editor } from "tinymce";

export type EditorProps = {
    authorized: boolean;
    visible: boolean;
    content?: string;
}

function downloadTinyMCE() {
    Promise.resolve(import(/* @vite-ignore */"https://cdn.tiny.cloud/1/" + tinymce_key + "/tinymce/6/tinymce.min.js"))
}

async function initTinyMCE(content?: string): Promise<Editor[]> {
    // const theme_toggle = useThemeToggle();
    console.log("TinyMCE is being initialized");

    return await tinymce.init({
        selector: "textarea#tinymce-editor",
        // content_css: '/tinymce/styles.css',
        height: 500,
        external_plugins: {
            tiny_mce_wiris: '/tinymce/math_wiris.min.js'
        },
        plugins: 'preview importcss searchreplace autolink autosave save directionality code visualblocks visualchars fullscreen image link media template codesample table charmap pagebreak nonbreaking anchor insertdatetime advlist lists wordcount help charmap quickbars emoticons',
        menubar: 'file edit view insert format tools table help',
        toolbar: 'addModalButton | undo redo | bold italic underline strikethrough | fontfamily fontsize blocks | alignleft aligncenter alignright alignjustify | outdent indent |  numlist bullist | forecolor backcolor removeformat | pagebreak | charmap emoticons | fullscreen  preview save print | insertfile image media template link anchor codesample | ltr rtl | tiny_mce_wiris_formulaEditor tiny_mce_wiris_formulaEditorChemistry',
        toolbar_sticky: true,
        toolbar_mode: 'sliding',
        contextmenu: 'link image table',
        draggable_modal: true,
        autosave_ask_before_unload: true,
        autosave_interval: '30s',
        skin: "oxide-dark",
        autosave_prefix: 'autosave-{path}{query}-{id}-',
        autosave_restore_when_empty: false,
        autosave_retention: '30s',
        image_advtab: true,
        image_caption: true,
        setup: (editor) => {
            editor.on("init", (args) => {
                console.log("TinyMCE is setup", editor, args, content)
                if (typeof content !== "undefined")
                    editor.setContent(content)
            })
        }
    })
}

export const [EditorInitializedProvider, useEditorInitialized] = createContextProvider(
    () => {
        const [isEditorInitialized, setIsEditorInitialized] = createSignal(false);

        return {
            isEditorInitialized, setIsEditorInitialized
        };
    }
);

const tinymce_key = "drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker";
const TinyMCE: Component<EditorProps> = (props) => {
    const editorInitialized = useEditorInitialized();
    const [editor, setEditor] = createSignal<Editor>();

    createEffect(() => {
        console.log("TinyMCE", {
            visible: props.visible,
            authed: props.authorized,
            init: editorInitialized?.isEditorInitialized()
        })

        if (props.authorized && props.visible) {
            if (!editorInitialized?.isEditorInitialized())
                downloadTinyMCE()

            setTimeout(async () => {
                const editors = await initTinyMCE(props.content)
                console.log(editors)
                setEditor(editors[0])
            }, 1000);
            editorInitialized?.setIsEditorInitialized(true)
        }
    })

    onCleanup(() => {
        editor()?.remove();
    })

    return (
        <textarea id="tinymce-editor" />
    )

}

export default TinyMCE;