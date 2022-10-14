import { AppShell, Button, Header, Navbar } from '@mantine/core'
import { Editor } from '@tinymce/tinymce-react'
import dynamic from 'next/dynamic'
import { useRef } from 'react'

const DynamicEditor: any = dynamic(() => import('@tinymce/tinymce-react').then(mod => mod.Editor) as any, {
    ssr: false,
});

const TinyMCE = () => {
    const editorRef = useRef<any | null>(null);

    const setup = (editorRaw: Editor) => {
        const editorClass: Editor = { editor: editorRaw } as any as Editor;
        const editor = editorClass.editor;
        if (typeof (editor) === 'undefined')
            return;

        editor.ui.registry.addButton('addModalButton', {
            text: 'Modal',
            onAction: () => {
                let elements = editor.contentDocument.querySelectorAll(".popup")
                let html = '<ul>'
                elements.forEach(elem => {
                    html += '<li>' + elem.tagName + '</li>'
                })
                html += '</ul>'

                editor.windowManager.open({
                    title: "Add modal",
                    body: {
                        type: 'panel',
                        items: [
                            {
                                type: 'htmlpanel',
                                html
                            }
                        ]

                    },
                    buttons: [
                        {
                            type: 'submit',
                            text: 'OK'
                        }
                    ]
                })
                // editor.insertContent('<button onclick="openModal()">Test</button>', { format: 'raw' });
            },
        });
    }

    return <>
        {/* <Button onClick={() => {
            const editorClass: Editor = { editor: editorRef.current } as any as Editor;
            const editor = editorClass.editor;
            if (typeof (editor) === 'undefined')
                return;

        }}>Test</Button> */}
        <DynamicEditor
            onInit={(_: any, editor: any) => {
                editorRef.current = editor;
            }}
            apiKey={'drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker'}
            initialValue=''
            init={{
                content_css: '/tinymce/styles.css',
                height: 500,
                external_plugins: {
                    tiny_mce_wiris: '/tinymce/math_wiris.min.js'
                },
                setup,
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
                image_caption: true
            }}
        />
    </>
}

export default TinyMCE
