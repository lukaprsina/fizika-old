// @refresh reload
import { createContextProvider } from "@solid-primitives/context";
import { usePrefersDark } from "@solid-primitives/media";
import { CookieOptions, cookieStorage, createStorage, StorageSetter } from "@solid-primitives/storage";
import { createEffect, createSignal, ParentComponent, Suspense } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Link,
  Meta, Routes,
  Scripts,
  Title
} from "solid-start";
import "./index.css";

export const AppShellHeader: ParentComponent = (props) => {
  return (
    <header
      class="w-full"
    >
      {props.children}
    </header>
  )
}

type ContentType = {
  fullWidth?: boolean;
}

export const AppShellContent: ParentComponent<ContentType> = (props) => {
  return (
    <div class="flex-grow max-w-5xl px-6 mx-auto" classList={{
      "w-full": props.fullWidth,
    }}>
      {props.children}
    </div>
  )
}

export const AppShellFooter: ParentComponent = (props) => {
  return (
    <footer
      class="w-full"
    >
      {props.children}
    </footer>
  )
}

export const [EditToggleProvider, useEditToggle] = createContextProvider(
  (props: { initial: boolean }) => {
    const [edit, setEdit] = createSignal(props.initial);

    return {
      edit,
      change: setEdit
    };
  }
);

type ThemeType = {
  dark: boolean;
  setCookies: StorageSetter<string, CookieOptions>
}

export const [ThemeToggleProvider, useThemeToggle] = createContextProvider(
  (props: ThemeType) => {
    const [dark, setDark] = createSignal(props.dark);

    createEffect(() => {
      if (dark()) {
        document.documentElement.classList.add('dark')
        props.setCookies("theme", "dark")
      }
      else {
        document.documentElement.classList.remove('dark')
        props.setCookies("theme", "light")
      }
    });

    setDark(props.dark)

    return {
      dark,
      setDark
    };
  }
);

type CookieType = {
  theme: "dark" | "light";
}

const AppShell: ParentComponent = (props) => {
  const [cookies, setCookies] = createStorage({
    api: cookieStorage,
    prefix: "fizika-scnm",
    options: {
      sameSite: "Lax",
    }
  })

  if (!cookies.theme) {
    const prefersDark = usePrefersDark();
    setCookies("theme", prefersDark() ? "dark" : "light");
  }

  return (
    <ThemeToggleProvider dark={cookies.theme == "dark"} setCookies={setCookies}>
      <EditToggleProvider initial={false}>
        <div class="flex min-h-screen flex-col dark:text-white dark:bg-neutral-900">
          {props.children}
        </div>
      </EditToggleProvider>
    </ThemeToggleProvider>
  )
}

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
            <AppShell>
              <Routes>
                <FileRoutes />
              </Routes>
            </AppShell>
          </Suspense>
        </ErrorBoundary>
        <Scripts />
      </Body>
    </Html>
  );
}
