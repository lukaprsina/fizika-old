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
            // base_url: "/tinymce",
            width: 800,
            height: 800,
            promotion: false,
            plugins: [
                'advlist', 'autolink', 'link', 'image', 'lists', 'charmap', 'preview', 'anchor', 'pagebreak',
                'searchreplace', 'wordcount', 'visualblocks', 'visualchars', 'code', 'fullscreen', 'insertdatetime',
                'media', 'table', 'emoticons', 'template', 'help'
            ],
            toolbar: 'undo redo | styles | bold italic | alignleft aligncenter alignright alignjustify | ' +
                'bullist numlist outdent indent | link image | print preview media fullscreen | ' +
                'forecolor backcolor emoticons | help',
            menu: {
                favs: { title: 'My Favorites', items: 'code visualaid | searchreplace | emoticons' }
            },
            menubar: 'favs file edit view insert format tools table help',
        })
    })

    return (
        <textarea id="tinymce-editor" />
    )

}

export default TinyMCE;