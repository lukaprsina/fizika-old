import { createEffect, createSignal, onMount, Show } from "solid-js";
import { useRouteData } from "solid-start";
import { withProtected } from "~/layouts/Protected";

export const { routeData, Page } = withProtected((user) => {
    const data = useRouteData<typeof routeData>();

    const [show, setShow] = createSignal(false);
    createEffect(() => {
        console.log("User", user.id, user.displayName)
        console.log("User Data", data().id, data().displayName)
        if (user.id) {
            setShow(true)
        }
    })

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

    return (
        <Show when={show()}>
            <textarea id="tinymce-editor" />
        </Show>
    )

})

export default Page;