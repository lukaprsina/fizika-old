import type { Component } from "solid-js";
import { createEffect, Show } from "solid-js"
import { useRouteData } from "solid-start"
import { createServerData$, redirect } from "solid-start/server"
import Footer from "~/components/Footer"
import Header from "~/components/Header"
import Markdown from "~/components/markdown"
import MonacoEditor from "~/components/MonacoEditor"
import SimpleMDE from "~/components/SimpleMDE";
import { AppShellHeader, AppShellContent, AppShellFooter } from "~/layouts/Providers"
import { authenticator } from "~/server/auth"

export const routeData = () => {
    return createServerData$(async (_, { request }) => {
        const user = await authenticator.isAuthenticated(request);
        if (!user) {
            console.log("Not logged in")
            throw redirect("/failure");
        } else {
            console.log("Logged in", user.displayName)
        }
        console.log("From server", { user })
        return user;
    }, {
        key: ["test"]
    });
};

const Edit: Component = () => {
    const current = useRouteData<typeof routeData>();

    return <>
        <AppShellHeader>
            <Header />
        </AppShellHeader>
        <AppShellContent>
            <Show when={current()}>
                <div
                    class="w-full h-full flex flex-row"
                >
                    <div class="w-1/2">
                        <SimpleMDE />
                    </div>
                    <div class="w-1/2">
                        <Markdown />
                    </div>
                </div>
            </Show>
        </AppShellContent>
        <AppShellFooter>
            <Footer />
        </AppShellFooter>
    </>
    // });
};

export default Edit;
