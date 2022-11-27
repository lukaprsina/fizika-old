import { ParentComponent } from "solid-js";
import { EditToggleProvider } from "~/routes/(home)";

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

const AppShell: ParentComponent = (props) => {
    return (
        <EditToggleProvider initial={false}>
            <div class="flex min-h-screen flex-col">
                {props.children}
            </div>
        </EditToggleProvider>
    )
}

export default AppShell