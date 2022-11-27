import { Component, createMemo, createSignal } from "solid-js"

const Editor: Component = () => {
    const [mce, setMce] = createSignal();

    createMemo(async () => {
        const flowers = "drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker";
        const mce = await import("tinymce")

        let manager = mce.default.EditorManager

        let ed = await mce.default.init({
            selector: "textarea#tinymce-editor",
            base_url: "/tinymce",
            width: 600,
            height: 300,
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

        /* const editor = new mce.Editor(
            "tinymce-editor",
            {

            },
            manager
        )

        setMce(editor) */
    })

    return (
        <textarea id="tinymce-editor"></textarea>
    )
}

export default Editor