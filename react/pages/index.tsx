import { AppShell, Button, Header, Navbar } from '@mantine/core'
import { Editor } from '@tinymce/tinymce-react'
import type { NextPage } from 'next'
import dynamic from 'next/dynamic'
import { useEffect, useRef, useState } from 'react'

const DynamicEditor = dynamic(() => import('@tinymce/tinymce-react').then(mod => mod.Editor), {
    ssr: false,
});

const Home: NextPage = () => {
    const editorRef = useRef<any | null>(null);

    const setup = (editorRaw: Editor) => {
        const editorClass: Editor = { editor: editorRaw } as any as Editor;
        const editor = editorClass.editor;
        if (typeof (editor) === 'undefined')
            return;

        editor.ui.registry.addButton('customInsertButton', {
            text: 'My Button',
            onAction: () => {
                editor.insertContent('<button onclick="console.log(1)" id="test" class="nextPage notProcessed">Test</button>', { format: 'raw' });
            },
        });
    }

    return (
        <AppShell
            padding="md"
            navbar={<Navbar width={{ base: 300 }} height={500} p="xs">{/* Navbar content */}</Navbar>}
            header={<Header height={60} p="xs">{/* Header content */}</Header>}
        >
            <Button onClick={() => {
                const editorClass: Editor = { editor: editorRef.current } as any as Editor;
                const editor = editorClass.editor;
                if (typeof (editor) === 'undefined')
                    return;

                console.log(editor.contentDocument.querySelector("button"))
            }}>Test</Button>
            <DynamicEditor
                onInit={(_, editor) => {
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
                    toolbar: 'customInsertButton | undo redo | bold italic underline strikethrough | fontfamily fontsize blocks | alignleft aligncenter alignright alignjustify | outdent indent |  numlist bullist | forecolor backcolor removeformat | pagebreak | charmap emoticons | fullscreen  preview save print | insertfile image media template link anchor codesample | ltr rtl | tiny_mce_wiris_formulaEditor tiny_mce_wiris_formulaEditorChemistry',
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
        </AppShell>
    )
}

export default Home
