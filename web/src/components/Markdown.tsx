import { compile, run } from "@mdx-js/mdx"
import type { Component, JSX, VoidComponent } from "solid-js";
import { getOwner, runWithOwner } from "solid-js";
import { createComponent, createSignal, Show } from "solid-js";
import { onMount } from "solid-js";
import * as jsx_runtime from 'solid-jsx'

const markdown = `<Test>Inside</Test>`

const Counter: VoidComponent = () => {
    const [count, setCount] = createSignal(0);

    return <>
        <p>{count()}</p>
        <button
            onClick={() => { setCount(count() + 1) }}
        >
            Add
        </button>
    </>
}

const Markdown: Component = () => {
    const [content, setContent] = createSignal<JSX.Element>();
    // const owner = getOwner();

    onMount(async () => {
        console.log("Shit")
        /* if (!owner)
            return; */

        const code = String(await compile(markdown, {
            outputFormat: 'function-body',
            jsxImportSource: 'solid-js',
            providerImportSource: 'solid-jsx',
        }))

        const Content = (await run(code, jsx_runtime)).default;
        /* runWithOwner(owner, () => {
            const component = createComponent(Content, {
                components: {
                    Test: () => <Counter />
                }
            })

            setContent(component)
        }) */

    })

    return <Show when={content}>
        <div>
            {content()}
        </div>
    </Show>
}

export default Markdown;