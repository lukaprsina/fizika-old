import { Component, For, Show } from "solid-js";
import { RouteDataArgs, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Content from "~/components/Content";
import Navbar from "~/components/Navbar";
import { Sidebar, SidebarItem } from "~/components/Sidebar";
import { prisma } from "~/server/db/client";
import { EditToggleProvider } from "../(home)";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topic_name]) => {
        const topic = await prisma.topic.findUnique({
            where: {
                title: topic_name
            }
        });

        if (!topic) return null;//throw new Error("Topic \"" + topic_name + "\" doesn't exist")

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
    const params = useParams();

    return (
        <EditToggleProvider initial={false}>
            <Navbar topic={params.topic} />
            <Content>
                <Sidebar>
                    <Show when={topics()}>
                        <For each={topics()}>{(topic, i) =>
                            <SidebarItem text={topic.title} href={topic.id.toString()} />
                        }
                        </For>
                    </Show>
                </Sidebar>
            </Content>
        </EditToggleProvider>
    )
}

export default TopicNavbar