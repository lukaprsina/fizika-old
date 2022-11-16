import { Component } from "solid-js"
import { RouteDataArgs, useParams, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import { prisma } from "~/server/db/client"

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async (_, { request }) => {
        const params = useParams();

        const topics = await prisma.page.findUnique({
            where: {

            }
        });

        return topics;
    })
}

const Course: Component = () => {
    const params = useParams();
    const page = useRouteData<typeof routeData>();
    return <p>{params.topic}: {params.page}</p>
}

export default Course