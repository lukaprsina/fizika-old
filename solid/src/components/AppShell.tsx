import { createContextProvider } from "@solid-primitives/context";
import { createEffect, createSignal, onMount, ParentComponent } from "solid-js";
import { usePrefersDark } from "@solid-primitives/media";

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

export const [ThemeToggleProvider, useThemeToggle] = createContextProvider(
    (props: { dark: boolean }) => {
        const [dark, setDark] = createSignal(props.dark);

        createEffect(() => {
            console.log(dark())
            if (dark())
                document.documentElement.classList.add('dark')
            else
                document.documentElement.classList.remove('dark')
        });

        onMount(async () => setDark(props.dark));

        return {
            dark,
            setDark
        };
    }
);

const AppShell: ParentComponent = (props) => {
    const prefersDark = usePrefersDark();

    return (
        <ThemeToggleProvider dark={prefersDark()}>
            <EditToggleProvider initial={false}>
                <div class="flex min-h-screen flex-col dark:text-white dark:bg-neutral-900">
                    {props.children}
                </div>
            </EditToggleProvider>
        </ThemeToggleProvider>
    )
}

export default AppShell