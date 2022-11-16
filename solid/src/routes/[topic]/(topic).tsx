import { Topic } from "@prisma/client";
import { Component, createEffect, For } from "solid-js"
import { RouteDataArgs, useParams, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import { prisma } from "~/server/db/client"

/* export function routeData2({ params }: RouteDataArgs) {
    return createServerData$(
        ([, id]) => prisma.students.findUnique({ where: { id } }),
        { key: () => ["students", params.id] }
    );
} */

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([a, topic_name]) => {
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
                title: true
            }
        });

        return pages;
    }, {
        key: () => ["page", params.topic]
    })
}

const Navbar: Component = () => {
    const topics = useRouteData<typeof routeData>();
    const params = useParams();

    createEffect(() => console.log(topics()))

    return <>
        <p>{params.topic}</p>
        <hr />
        <div>
            <For each={topics()}>{(topic, i) =>
                <NavbarItem topic={topic} />
            }
            </For>
        </div>
    </>
}

type NavbarItemType = {
    topic: { title: string };
}

const NavbarItem: Component<NavbarItemType> = (props) => {
    return <p>{props.topic.title}</p>
}

export default Navbar