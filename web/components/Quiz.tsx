import { NodeViewWrapper, NodeViewWrapperProps, ReactNodeViewRenderer } from '@tiptap/react'
import { mergeAttributes, Node } from '@tiptap/core'

const Quiz = (props: NodeViewWrapperProps) => {
    const increase = () => {
        props.updateAttributes({
            count: props.node.attrs.count + 1,
        })
    }

    return (
        <NodeViewWrapper className="react-component">
            <span className="label">React Component</span>

            <div className="content">
                <button onClick={increase}>
                    Like je bil smashan {props.node.attrs.count}-krat.
                </button>
            </div>
        </NodeViewWrapper>
    )
}

const QuizNode = Node.create({
    name: 'reactComponent',
    group: 'block',
    atom: true,

    addAttributes() {
        return {
            count: {
                default: 0,
            },
        }
    },

    parseHTML() {
        return [
            {
                tag: 'react-component',
            },
        ]
    },

    renderHTML({ HTMLAttributes }) {
        return ['react-component', mergeAttributes(HTMLAttributes)]
    },

    addNodeView() {
        return ReactNodeViewRenderer(Quiz)
    },
})


export default QuizNode