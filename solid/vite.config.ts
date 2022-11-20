import solid from "solid-start/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [solid()],
  build: {
    minify: "terser",
  },
  ssr: { external: ["@prisma/client"] },
});
