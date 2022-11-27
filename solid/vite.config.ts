import solid from "solid-start/vite";
import { defineConfig } from "vite";
import { viteStaticCopy } from 'vite-plugin-static-copy'

export default defineConfig({
  plugins: [
    solid(),
    viteStaticCopy({
      targets: [
        {
          src: "node_modules/tinymce/",
          dest: ""
        }
      ]
    })
  ],
  build: {
    minify: "terser",
  },
  ssr: { external: ["@prisma/client"] },
});
