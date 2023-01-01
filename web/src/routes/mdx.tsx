import type { Component } from "solid-js";
import { onMount } from "solid-js"
import { compile, run } from '@mdx-js/mdx'
import * as runtime from 'solid-js/jsx-runtime'

const MDX = () => {
    onMount(async () => {
        const code = String(await compile(
            '# hi',
            {
                outputFormat: 'function-body',
                jsxImportSource: 'solid-js'
            }
        ))

        console.log(code)
        const { default: Content } = await run(code, runtime);
        console.log(Content.toString())

        const a = <Content />
    })

    return <>lol</>
}

export default MDX
