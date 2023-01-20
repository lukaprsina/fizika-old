import { Component, Show, For } from "solid-js"
import Footer from "~/components/Footer"
import Header from "~/components/Header"
import MonacoEditor from "~/components/MonacoEditor"
import { withProtected } from "~/layouts/Protected"
import { AppShellHeader, AppShellContent, AppShellFooter } from "~/layouts/Providers"

export const { Page, routeData } = withProtected((user) => {
    return <>
        <AppShellHeader>
            <Header />
        </AppShellHeader>
        <AppShellContent>
            <MonacoEditor
                active={true}
                user={user}
            />
        </AppShellContent>
        <AppShellFooter>
            <Footer />
        </AppShellFooter>
    </>
});

/* export default function () {
    return <p>Test</p>
} */