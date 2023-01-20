import type { Component, JSX, ParentComponent, ParentProps } from "solid-js";
import { createComponent, useContext } from "solid-js";
import { Show, createSignal } from "solid-js";
import { onMount } from "solid-js"
import { compile, run } from '@mdx-js/mdx'
import * as jsx_runtime from 'solid-jsx'
import { MDXContext } from 'solid-jsx'

const text = `{console.log(Test)}
# Heading (rank 1)
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

const MDX: Component = () => {
    const [Content, setContent] = createSignal<JSX.Element>();



    return (
        
    )
}

const H1Test: ParentComponent = (props) => {
    return <h4>{props.children}</h4>
}

export default MDX