// @refresh reload
import { Suspense } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  Routes,
  Scripts,
  Title
} from "solid-start";
import Providers from "./layouts/Providers";
import "./root.css";

export default function Root() {
  const tinymce_key = "drmp13ceee93lq23r1dankva2b57mbl7wnpr2b4u9et8nker";

  return (
    <Html lang="en">
      <Head>
        <Title>Create JD App</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <script
          src={"https://cdn.tiny.cloud/1/" + tinymce_key + "/tinymce/6/tinymce.min.js"}
          referrerpolicy="origin"
          defer
        />
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
