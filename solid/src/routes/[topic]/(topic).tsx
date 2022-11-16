import { Topic } from "@prisma/client";
import { Component, createEffect, For } from "solid-js"
import { A, RouteDataArgs, useParams, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import { Navbar, NavbarItem } from "~/components/Navbar";
import { prisma } from "~/server/db/client"

/* export function routeData2({ params }: RouteDataArgs) {
    return createServerData$(
        ([, id]) => prisma.students.findUnique({ where: { id } }),
        { key: () => ["students", params.id] }
    );
} */

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topic_name]) => {
        const topic = await prisma.topic.findUnique({
            where: {
                title: topic_name
            }
        });

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

    return <>
        <p>{params.topic}</p>
        <hr />
        <Navbar>
            <For each={topics()}>{(topic, i) =>
                <NavbarItem text={topic.title} href={topic.id.toString()} />
            }
            </For>
        </Navbar>
    </>
}

export default TopicNavbar