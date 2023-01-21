// import type { } from "@mdx-js/mdx"
import { compile, run } from "@mdx-js/mdx"
import type { Component, JSX } from "solid-js";
import { createComponent, createSignal, Show } from "solid-js";
import { onMount } from "solid-js";
import * as jsx_runtime from 'solid-jsx'

const markdown = `<Test>Inside</Test>`

const MDXOg: Component = () => {
    const [content, setContent] = createSignal<JSX.Element>();

    onMount(async () => {
        const code = String(await compile(markdown, {
            outputFormat: 'function-body',
            jsxImportSource: 'solid-js',
            providerImportSource: 'solid-jsx',
        }))

        const Content = (await run(code, jsx_runtime)).default;
        const component = createComponent(Content,
            {
                components: {
                    Test: () => <span>Pluto</span>
                }
            })

        setContent(component)
        console.log({ Content, component })
    })

    return <Show when={content}>
        {content()}
    </Show>
}

export default MDXOg;