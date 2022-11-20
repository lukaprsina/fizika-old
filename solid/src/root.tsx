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
  Title,
} from "solid-start";
import "./index.css";

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>Fizka</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="description" content="Razlaga in vaje za srednješolski nivo fizike. Narejeno v šolskem centru Novo mesto." />
        <Link rel="manifest" href="/manifest.webmanifest" />
      </Head>
      <Body>
        <ErrorBoundary>
          <Suspense fallback={<div>Nalagam ...</div>}>
            <Routes>
              <FileRoutes />
            </Routes>
          </Suspense>
        </ErrorBoundary>
        <Scripts />
      </Body>
    </Html>
  );
}
