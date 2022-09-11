import React from 'react';
import { EditorState, Modifier } from 'draft-js';
import { NextPage } from 'next';
import dynamic from 'next/dynamic';

const NoSSREditor = dynamic(() =>
    import('react-draft-wysiwyg').then((mod) => mod.Editor), {
    ssr: false
})

const WYSIWYG: NextPage = () => {
    return <NoSSREditor
        wrapperClassName="demo-wrapper"
        editorClassName="demo-editor"
        // eslint-disable-next-line react/jsx-key
        toolbarCustomButtons={[<CustomOption />]}
    />
}

type CustomOptionProps = {
    editorState: EditorState;
    onChange: any;
}

const CustomOption = ({ editorState, onChange }: CustomOptionProps) => {
    return <div onClick={() => {
        const contentState = Modifier.replaceText(
            editorState.getCurrentContent(),
            editorState.getSelection(),
            '⭐',
            editorState.getCurrentInlineStyle(),
        );
        onChange(EditorState.push(editorState, contentState, 'insert-characters'));
    }}>E</div>
}

export default WYSIWYG;