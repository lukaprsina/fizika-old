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

//console.log({arguments, props})

const hardcoded = `/*@jsxRuntime automatic @jsxImportSource solid-js*/
const {jsx: _jsx} = arguments[0];
const {useMDXComponents: _provideComponents} = arguments[0];
function _createMdxContent(props) {
    console.log({arguments, props})
  const {Test} = Object.assign({}, _provideComponents(), props.components);
  if (!Test) _missingMdxReference("Test", true);
  return _jsx(Test, {
    children: "OMG"
  });
}
function MDXContent(props = {}) {
  const {wrapper: MDXLayout} = Object.assign({}, _provideComponents(), props.components);
  return MDXLayout ? _jsx(MDXLayout, Object.assign({}, props, {
    children: _jsx(_createMdxContent, props)
  })) : _createMdxContent(props);
}
return {
  default: MDXContent
};
function _missingMdxReference(id, component) {
  throw new Error("Expected " + (component ? "component" : "object") + " \`" + id + "\` to be defined: you likely forgot to import, pass, or provide it.");
}`

const MDX: Component = () => {
    const [Content, setContent] = createSignal<JSX.Element>();

    onMount(async () => {
        const code = String(await compile(
            "<Test>OMG</Test>",
            // "# test",
            {
                outputFormat: 'function-body',
                jsxImportSource: 'solid-js',
                providerImportSource: 'solid-jsx',
                // development: true,
                // format: 'mdx',
                // useDynamicImport: true,
            }
        ))

        console.log("code", code)
        const JSXContent = (await run(hardcoded, jsx_runtime)).default;
        console.log("content", JSXContent.toString())

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

export const MDXProvider = (
    properties: ParentProps<{
        components: {
            [k: string]: (properties_: ParentProps) => JSX.Element;
        };
    }>
): JSX.Element => {
    const func = createComponent(MDXContext.Provider, {
        value: {
            ...useContext(MDXContext),
            ...properties.components,
        },
        children: () => properties.children,
    });
    console.log("LOL", func)
    return func
}

const H1Test: ParentComponent = (props) => {
    return <h4>{props.children}</h4>
}

export default MDX