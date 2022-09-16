import { AppShell, Header, Navbar } from '@mantine/core'
import { Editor } from '@tinymce/tinymce-react'
import type { NextPage } from 'next'
import dynamic from 'next/dynamic'
import { useRef } from 'react'

const DynamicHeader = dynamic(() => import('@tinymce/tinymce-react').then(mod => mod.Editor), {
  ssr: false,
});

const Home: NextPage = () => {
  const editorRef = useRef<any | null>(null);

  return (
    <AppShell
      padding="md"
      navbar={<Navbar width={{ base: 300 }} height={500} p="xs">{/* Navbar content */}</Navbar>}
      header={<Header height={60} p="xs">{/* Header content */}</Header>}
    >
      <DynamicHeader
        onInit={(_, editor) => {
          editorRef.current = editor;
        }}
        apiKey={'drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker'}
        initialValue="<p>This is the initial content of the editor.</p>"
        init={{
          height: 500,
          menubar: false,
          external_plugins: {
            tiny_mce_wiris: '/plugins/tiny_mce_wiris.min.js'
          },
          toolbar: 'tiny_mce_wiris_formulaEditor tiny_mce_wiris_formulaEditorChemistry',
          draggable_modal: true,
        }}
      />
    </AppShell>
  )
}

export default Home
