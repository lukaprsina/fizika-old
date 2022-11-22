import { Component, For, Show } from "solid-js";
import { RouteDataArgs, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import { AppShellContent, AppShellHeader } from "~/root";
import Header from "~/components/layout/Header";
import { Sidebar, SidebarItem } from "~/components/layout/Sidebar";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topic_name]) => {
        const topic = await prisma.topic.findUnique({
            where: {
                title: topic_name
            }
        });

        if (!topic) return null;

        const pages = await prisma.page.findMany({
            where: {
                topicId: topic.id
            },
            select: {
                title: true,
                id: true
            }
        });

        return pages;
    }, {
        key: () => ["topic", params.topic]
    })
}

const TopicNavbar: Component = () => {
    const topics = useRouteData<typeof routeData>();

    return <>
        <AppShellHeader>
            <Header />
        </AppShellHeader>
        <AppShellContent>
            <Sidebar>
                <Show when={topics()}>
                    <For each={topics()}>{(topic, i) =>
                        <SidebarItem text={topic.title} href={topic.id.toString()} />
                    }
                    </For>
                </Show>
            </Sidebar>
        </AppShellContent>
    </>
}

export default TopicNavbar