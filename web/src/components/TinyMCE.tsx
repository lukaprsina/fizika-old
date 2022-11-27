import type { Component } from "solid-js";
import { onMount } from "solid-js";
import { withProtected } from "~/layouts/Protected";

const Editor: Component = withProtected((user) => {
    onMount(async () => {
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
    console.log("Rendering tinymce")

    return <>
        <textarea id="tinymce-editor" />
    </>

})

export default Editor