import solid from "solid-start/vite";
import { defineConfig } from "vite";
import dotenv from "dotenv";
import { viteStaticCopy } from "vite-plugin-static-copy";
import mdx from '@mdx-js/rollup';
import remarkGfm from 'remark-gfm';

export default defineConfig(() => {
  dotenv.config();
  return {
    plugins: [
      mdx({ jsxImportSource: 'solid-jsx', remarkPlugins: [remarkGfm] }),
      solid({ /* ssr: true */ }),
      viteStaticCopy({
        targets: [
          {
            src: "node_modules/@wiris/mathtype-tinymce6/plugin.min.js",
            dest: "tinymce",
            rename: "math_wiris.min.js"
          }
        ]
      })
    ],
    ssr: {
      external: ['@prisma/client'],
    },
  };
});
