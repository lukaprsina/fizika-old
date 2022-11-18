import { createContextProvider } from "@solid-primitives/context";
import { Component, createSignal, For, Show } from "solid-js";
import { useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Content from "~/components/Content";
import Navbar from "~/components/Navbar";
import { Sidebar, SidebarItem } from "~/components/Sidebar";
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
        <EditToggleProvider initial={false}>
            <main class="w-screen">
                <Navbar />
                <Content>
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
                </Content>
            </main>
        </EditToggleProvider>
    )
}

export default Home