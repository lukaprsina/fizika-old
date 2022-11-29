import type { Component } from "solid-js";
import { For, Show } from "solid-js";
import type { RouteDataArgs } from "solid-start";
import { useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Footer from "~/components/layout/Footer";
import Header from "~/components/layout/Header";
import { Sidebar, SidebarItem } from "~/components/layout/Sidebar";
import { AppShellContent, AppShellFooter, AppShellHeader } from "~/layouts/Providers";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([, topic_name]) => {
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
                    <For each={topics()}>{(topic) =>
                        <SidebarItem text={topic.title ?? ""} href={topic.id.toString()} />
                    }
                    </For>
                </Show>
            </Sidebar>
        </AppShellContent>
        <AppShellFooter>
            <Footer />
        </AppShellFooter>
    </>
}

export default TopicNavbar