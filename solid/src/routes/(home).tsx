import { createContextProvider } from "@solid-primitives/context";
import { Component, createSignal, For, Show } from "solid-js";
import { useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import { AppShellContent, AppShellHeader } from "~/root";
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

const Home: Component = () => {
    const topics = useRouteData<typeof routeData>();

    return <>
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
    </>
}

export default Home