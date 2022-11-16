import { Component } from "solid-js"
import { RouteDataArgs, useParams, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import { prisma } from "~/server/db/client"

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, args]) => {
        console.log(args)
        if (typeof args === "string") throw "Args is string";

        const topic = await prisma.topic.findUnique({
            where: {
                title: args.topic
            }
        });

        const page_id = parseInt(args.page);

        if (isNaN(page_id)) throw "Page ID is NaN";

        const pages = await prisma.page.findUnique({
            where: {
                topicId_id: {
                    id: page_id,
                    topicId: topic.id,
                }
            }
        });

        return pages;
    }, {
        key: () => ["page", { topic: params.topic, page: params.page }]
    })
}

const PageNavbar: Component = () => {
    const page = useRouteData<typeof routeData>();
    const params = useParams();

    if (typeof page() === "undefined") {
        return <p>What</p>
    }

    return <>
        <p>{params.topic}</p>
        <hr />
        <div innerHTML={page().html}></div>
    </>
}

export default PageNavbar