import React, { useEffect, useRef } from 'react';
import { EditorState, Modifier } from 'draft-js';
import { NextPage } from 'next';
import dynamic from 'next/dynamic';
import 'react-draft-wysiwyg/dist/react-draft-wysiwyg.css'
// import { MathfieldElement } from 'mathlive';

const NoSSREditor = dynamic(() =>
    import('react-draft-wysiwyg').then((mod) => mod.Editor), {
    ssr: false
})

let mathRendered = false;

const WYSIWYG: NextPage = () => {
    const mathRef = useRef<any>();
    useEffect(() => {
        const a = async () => {
            const mathlive = (await import('mathlive'))

            let mfe = new mathlive.MathfieldElement();

            mfe.setOptions({
                virtualKeyboards: "all",
                virtualKeyboardMode: "onfocus",
                fontsDirectory: "/assets/fonts",
                soundsDirectory: "/assets/sounds",
            })

            mfe.value = "x=\\frac{-b\\pm\\sqrt{b ^ 2 - 4ac}}{2a}";
            // mfe.executeCommand(['switchMode', 'math'])

            if (!mathRendered && typeof mathRef.current !== 'undefined') {
                mathRef.current.appendChild(mfe)
                mathRendered = true;
            }
        }

        a()
    }, [])

    return <>
        <NoSSREditor
            wrapperClassName="demo-wrapper"
            editorClassName="demo-editor"
            // eslint-disable-next-line react/jsx-key
            toolbarCustomButtons={[<CustomOption />]}
        />
        <div ref={mathRef}></div>

    </>
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