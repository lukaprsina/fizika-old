import type { Component, JSX, ParentComponent } from "solid-js";
import { Show, createSignal } from "solid-js";
import { onMount } from "solid-js"
import { compile, run } from '@mdx-js/mdx'
import * as jsx_runtime from 'solid-jsx'
import { MDXProvider } from 'solid-jsx'

const MDX: Component = () => {
    const [Content, setContent] = createSignal<JSX.Element>();

    onMount(async () => {
        const text = `# Heading (rank 1)
## Heading 2
### 3
#### 4
##### 5
###### 6

> Block quote

* Unordered
* List

1. Ordered
2. List

A paragraph, introducing a thematic break:

---

\`\`\`js
some.code()
\`\`\`

<Test>OMG</Test>

a [link](https://example.com), an ![image](./image.png), some *emphasis*,
something **strong**, and finally a little \`code()\`.
        `;

        const code = String(await compile(
            text,
            {
                outputFormat: 'function-body',
                jsxImportSource: 'solid-jsx',
                // providerImportSource: 'solid-jsx',
                // format: 'mdx',
                // useDynamicImport: true,
                // development: true,
            }
        ))

        const JSXContent = (await run(code, jsx_runtime)).default;
        setContent(JSXContent)
    })

    return (
        <MDXProvider
            components={{
                ["Test"]: H1Test
            }}
        >
            <Show when={Content}>
                {Content()}
            </Show>
        </MDXProvider>
    )
}

const H1Test: ParentComponent = (props) => {
    return <h4>{props.children}</h4>
}

export default MDX
