import { NextPage } from "next";
import { Button, ButtonGroup } from '@mui/material'
import {
    useEditor,
    EditorContent,
    BubbleMenu,
    FloatingMenu,
    Editor,
} from '@tiptap/react'
import Quiz from "../components/Quiz"
import Highlight from '@tiptap/extension-highlight'
import Typography from '@tiptap/extension-typography'
import Table from '@tiptap/extension-table'
import TableCell from '@tiptap/extension-table-cell'
import TableHeader from '@tiptap/extension-table-header'
import TableRow from '@tiptap/extension-table-row'
import StarterKit from '@tiptap/starter-kit'

let html = `
    <div class="eplxSlide popupImpl" id="c1f02e3b65b72ceaf9d61c167e5c5a98" name="starigrki">
        <react-component count="1"></react-component>
        <div class="popupContent">
            <h1>Dosežki starih Grkov</h1>
            <p>Na podlagi opazovanj so ugotovili:</p>
            <ul id="a2d1e02aafe6e351babc5cf3aae94798">
                <li>Zemlja je okrogla, ker ima Zemljina senca ob luninih mrkih okrogel obris (Aristotel, 330 pr.n.št.).</li>
                <li>280-260 let pred našim štetjem je filozof Aristarh trdil, da se Zemlja giblje okoli Sonca in da za celotno pot potrebuje 365 dni, ostale zvezde pa so (neskončno) daleč stran. Žal ni imel dovolj tehtnih dokazov in somišljenikov, tako da so se Grki vrnili k ideji o osrednjem mestu Zemlje v vesolju.</li>
                <li>Eratosten (240 pr.n.št.) je določil razmerje med velikostjo Zemlje  in Lune. Njegovi oceni obsega Zemlje znašata 39 690 km in 45 007 km (dejanski obseg Zemlje okrog Ekvatorja je približno 40 000 km). Skušal je uvesti leto z 365,25 dneva.</li>
                <li>Hiparh (150 pr.n.št.) je napisal katalog 1022 zvezd, razporejenih na 6 magnitud. Opisal je precesijo Zemlje in določil oddaljenost med Zemljo in Luno.</li>
                <li><b id="b5c3f7b829f7d2ddd0151b8d384602b2">Ptolomej</b> (150 n.št.) uvede geocentrični sistem osončja.</li>
            </ul>
            <table class="centered">
                <tbody><tr><td>
                    <table id="4de8e2881e86a0775e161e0753171c4e">
                        <tbody><tr><td>
                            <img style="height:150px;" title="Lunin mrk." src="lunin-mrk.PNG" alt="(lunin-mrk.PNG)">
                        </td></tr>
                        </tbody><caption class="imageCaption" align="bottom">Lunin mrk.</caption>
                    </table>

                </td><td>
                        <table id="3ad5c773408e55b3b76eea058aeddcb6">
                            <tbody><tr><td>
                                <img style="height:150px;" src="aristotel1.png" alt="(aristotel1.png)" title="Zemlja in planeti krožijo okoli Sonca.">
                            </td></tr>
                            </tbody><caption class="imageCaption" align="bottom">Zemlja in planeti krožijo okoli Sonca.</caption>
                        </table>

                    </td><td>
                        <table id="984a29ee319d76b5fa7481444f1e7bc5">
                            <tbody><tr><td>
                                <img alt="(eratosten1.gif)" title="Določitev obsega Zemlje." src="eratosten1.gif" style="height:150px;">
                            </td></tr>
                            </tbody><caption align="bottom" class="imageCaption">Določitev obsega Zemlje.</caption>
                        </table>

                    </td><td>
                        <table id="7a325dd1d95393514192ef3c3adf92f5">
                            <tbody><tr><td>
                                <img src="hipah1.gif" title="Precesijski cikel Zemlje." alt="(hipah1.gif)" style="height:150px;">
                            </td></tr>
                            </tbody><caption class="imageCaption" align="bottom">Precesijski cikel Zemlje.</caption>
                        </table>

                    </td><td>
                        <table id="11bbe715e71e4e3ec6745cc5b0ff2e7e">
                            <tbody><tr><td>
                                <img title="Geocentrični sistem." src="Ptolomej-sistem-Bartolomeu_Velho_1568.jpg" alt="(Ptolomej-sistem-Bartolomeu_Velho_1568.jpg)" style="height:150px;">
                            </td></tr>
                            </tbody><caption class="imageCaption" align="bottom">Geocentrični sistem.</caption>
                        </table>

                    </td></tr>
                </tbody></table>
            <p><a class="button-blue button  close" href="#">Zapri</a></p>
        </div>
    </div>
`

const TipTap: NextPage = () => {
    const editor = useEditor({
        extensions: [
            StarterKit,
            Highlight,
            Typography,
            Quiz
        ],
        content: html,
        onUpdate: ({ editor: tiptap, transaction }) => {
            console.log(tiptap.getJSON())
        }
    })

    editor?.commands.setMedia

    return (
        <div>
            {editor && <BubbleMenu className="bubble-menu" tippyOptions={{ duration: 100 }} editor={editor}>
                <ButtonGroup variant="contained" size="small">
                    <Button onClick={() => editor.chain().focus().toggleBold().run()}>
                        Bold
                    </Button>
                    <Button onClick={() => editor.chain().focus().toggleItalic().run()}>
                        Italic
                    </Button>
                    <Button onClick={() => editor.chain().focus().toggleStrike().run()}>
                        Strike
                    </Button>
                </ButtonGroup>
            </BubbleMenu>}

            {editor && <FloatingMenu className="floating-menu" tippyOptions={{ duration: 100 }} editor={editor}>
                <ButtonGroup variant="contained" size="small">
                    <Button onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}>
                        H1
                    </Button>
                    <Button onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}>
                        H2
                    </Button>
                    <Button onClick={() => editor.chain().focus().toggleBulletList().run()}>
                        Bullet List
                    </Button>
                </ButtonGroup>
            </FloatingMenu >}
            <EditorContent
                onChange={(event) => {
                    console.log(event)
                }}
                editor={editor}
            />
        </div>
    )
}

export default TipTap