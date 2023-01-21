import { compile, run } from "@mdx-js/mdx"
import type { Component, JSX, Owner, VoidComponent } from "solid-js";
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
        >Add</button>
    </>
}

const MDX: Component = () => {
    const [content, setContent] = createSignal<JSX.Element>();
    const [owner, setOwner] = createSignal<Owner | null>(getOwner());

    onMount(async () => {
        if (!owner())
            return;

        const code = String(await compile(markdown, {
            outputFormat: 'function-body',
            jsxImportSource: 'solid-js',
            providerImportSource: 'solid-jsx',
        }))

        const Content = (await run(code, jsx_runtime)).default;
        runWithOwner(owner() as Owner, () => {
            const component = createComponent(Content, {
                components: {
                    Test: () => <Counter />
                }
            })

            setContent(component)
        })

    })

    return <Show when={content}>
        {content()}
    </Show>
}

export default MDX;