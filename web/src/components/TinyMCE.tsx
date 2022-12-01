import type { Component } from "solid-js";
import { onMount } from "solid-js";

export type EditorProps = {
    show: boolean;
}

const TinyMCE: Component<EditorProps> = (props) => {
    const tinymce_key = "drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker";
    console.log("Whole")
    onMount(() => {
        console.log("Sync Mount")
    })

    onMount(async () => {
        if (!props.show) {
            console.log("TinyMCE hidden")
            return;
        } else
            console.log("Init TinyMCE")

        await import(/* @vite-ignore */"https://cdn.tiny.cloud/1/" + tinymce_key + "/tinymce/6/tinymce.min.js")
        await tinymce.init({
            selector: "textarea#tinymce-editor",
            content_css: '/tinymce/styles.css',
            height: 500,
            external_plugins: {
                tiny_mce_wiris: '/tinymce/math_wiris.min.js'
            },
            extended_valid_elements: "button[*]",
            plugins: 'preview importcss searchreplace autolink autosave save directionality code visualblocks visualchars fullscreen image link media template codesample table charmap pagebreak nonbreaking anchor insertdatetime advlist lists wordcount help charmap quickbars emoticons',
            menubar: 'file edit view insert format tools table help',
            toolbar: 'addModalButton | undo redo | bold italic underline strikethrough | fontfamily fontsize blocks | alignleft aligncenter alignright alignjustify | outdent indent |  numlist bullist | forecolor backcolor removeformat | pagebreak | charmap emoticons | fullscreen  preview save print | insertfile image media template link anchor codesample | ltr rtl | tiny_mce_wiris_formulaEditor tiny_mce_wiris_formulaEditorChemistry',
            toolbar_sticky: true,
            toolbar_mode: 'sliding',
            contextmenu: 'link image table',
            draggable_modal: true,
            autosave_ask_before_unload: true,
            autosave_interval: '30s',
            autosave_prefix: '{path}{query}-{id}-',
            autosave_restore_when_empty: false,
            autosave_retention: '2m',
            image_advtab: true,
            image_caption: true,
        })
    })

    return (
        <textarea id="tinymce-editor" />
    )

}

export default TinyMCE;