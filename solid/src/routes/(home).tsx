import { createContextProvider } from "@solid-primitives/context";
import { Component, createSignal, For, Show } from "solid-js";
import { useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import AppShell, { AppShellContent, AppShellHeader } from "~/components/AppShell";
import Header from "~/components/layout/Header";
import { Sidebar, SidebarItem } from "~/components/layout/Sidebar";
import { prisma } from "~/server/db/client";

export function routeData() {
    return createServerData$(async (_, { request }) => {
        const topics = await prisma.topic.findMany({
            where: {
                course: { title: "Fizika" }
            },
            include: {
                authors: {}
            },
            orderBy: { year: "asc" },
        });

        return topics;
    })
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

const Home: Component = () => {
    const topics = useRouteData<typeof routeData>();

    return (
        <AppShell>
            <AppShellHeader>
                <Header />
            </AppShellHeader>
            <AppShellContent>
                <Show when={topics()}>
                    <Sidebar>
                        <For each={topics()}>{(topic, i) =>
                            <SidebarItem
                                text={topic.title}
                            />
                        }
                        </For>
                    </Sidebar>
                </Show>
            </AppShellContent>
        </AppShell>
    )
}

export default Home