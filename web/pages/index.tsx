import type { NextPage } from 'next'
import { useState } from 'react';
import { Button, Paper } from '@mui/material';
import dynamic from 'next/dynamic';
import initialState from '../components/initialState'

import { EditorState, convertFromRaw, convertToRaw, Editor } from 'draft-js';
import 'draft-js/dist/Draft.css';

import { composeDecorators } from '@draft-js-plugins/editor';

import createMathjaxPlugin from 'draft-js-mathjax-plugin'

import createImagePlugin from '@draft-js-plugins/image';

// import createAlignmentPlugin from '@draft-js-plugins/alignment';

import createFocusPlugin from '@draft-js-plugins/focus';

// import createResizeablePlugin from '@draft-js-plugins/resizeable';

import createBlockDndPlugin from '@draft-js-plugins/drag-n-drop';

// import createDragNDropUploadPlugin from '@draft-js-plugins/drag-n-drop-upload';

// import mockUpload from '../components/MockUpload';
import { Box } from '@mui/system';

const focusPlugin = createFocusPlugin();
// const resizeablePlugin = createResizeablePlugin();
const blockDndPlugin = createBlockDndPlugin();
// const alignmentPlugin = createAlignmentPlugin();
// const { AlignmentTool } = alignmentPlugin;
const mathjaxPlugin = typeof (window) !== 'undefined' ? createMathjaxPlugin({

}) : null;

const decorator = composeDecorators(
  /* resizeablePlugin.decorator,
  alignmentPlugin.decorator, */
  focusPlugin.decorator,
  blockDndPlugin.decorator
);

const imagePlugin = createImagePlugin({ decorator });

/* const dragNDropFileUploadPlugin = createDragNDropUploadPlugin({
  handleUpload: mockUpload,
  addImage: imagePlugin.addImage,
}); */

const plugins = [
  blockDndPlugin,
  focusPlugin,
  imagePlugin,
  mathjaxPlugin
  /* dragNDropFileUploadPlugin,
  alignmentPlugin,
  resizeablePlugin, */
];

const NoSSREditor = dynamic(() =>
  import('@draft-js-plugins/editor'), {
  ssr: false
})

const Home: NextPage = () => {
  const [editorState, setEditorState] = useState(
    () => EditorState.createWithContent(convertFromRaw(initialState))
  );

  const onChange = (state: EditorState) => {
    setEditorState(state)
  }

  // const [editor, setEditor] = useState<Editor | null>(null);

  return <Box
    sx={{
      width: "100%",
      marginY: "50px",
    }}
  >
    <Paper
      // onClick={() => { editor?.focus() }}
      elevation={2}
      sx={{
        maxWidth: "800px",
        margin: "auto",
        padding: "20px"
      }}
    >
      <NoSSREditor
        editorState={editorState}
        onChange={onChange}
        plugins={plugins}
      // ref={(element: any) => { setEditor(element) }}
      />
      {/* <AlignmentTool /> */}
      <Button
        onClick={async () => {
          const contentState = editorState.getCurrentContent();
          const raw = convertToRaw(contentState);

          let response = await fetch("/api/hello", {
            body: JSON.stringify(raw),
            headers: {
              'Access-Control-Allow-Origin': '*',
              'Content-Type': 'application/json',
            },
            method: 'POST'
          })

          console.log({ response })
        }}
      >Send</Button>
    </Paper>
  </Box>
}

export default Home
