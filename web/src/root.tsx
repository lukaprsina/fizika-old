// @refresh reload
import { createEffect, createSignal, Show, Suspense } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Link,
  Meta,
  Routes,
  Scripts,
  Title
} from "solid-start";
import Providers from "./layouts/Providers";
import "./root.css";

export const [loadTinyMCE, setLoadTinyMCE] = createSignal(false);

export default function Root() {
  const tinymce_key = "drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker";
  createEffect(() => console.log("loadTinyMCE", loadTinyMCE()))
  return (
    <Html lang="en">
      <Head>
        <Title>Fizka</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="description" content="Razlaga in vaje za srednješolski nivo fizike. Narejeno v šolskem centru Novo mesto." />
        <Link rel="manifest" href="/manifest.webmanifest" />
        <Show when={loadTinyMCE()}>
          {"Test"}
          <script
            onLoad={() => console.log("Loaded script")}
            src={"https://cdn.tiny.cloud/1/" + tinymce_key + "/tinymce/6/tinymce.min.js"}
            referrerpolicy="origin"
          />
        </Show>
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <Providers>
              <Routes>
                <FileRoutes />
              </Routes>
            </Providers>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html >
  );
}
