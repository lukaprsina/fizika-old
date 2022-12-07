// @refresh reload
import { Suspense } from "solid-js";
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

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>Fizika</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="description" content="Razlaga in vaje za srednješolski nivo fizike. Narejeno v šolskem centru Novo mesto." />
        <Link rel="manifest" href="/manifest.webmanifest" />
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
