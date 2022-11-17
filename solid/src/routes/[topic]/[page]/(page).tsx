import { Component } from "solid-js";
import { RouteDataArgs, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topicArg, pageArg]) => {
        console.log({ topicArg, pageArg })
        const topic = await prisma.topic.findUnique({
            where: {
                title: topicArg
            }
        });

        const page_id = parseInt(pageArg);

        console.log({ page_id })
        if (isNaN(page_id)) throw "Page ID is NaN";
        console.log("After pageID")

        const page = await prisma.page.findUnique({
            where: {
                topicId_id: {
                    id: page_id,
                    topicId: topic.id,
                }
            }
        });

        console.log("Page:", page.title)

        return page;
    }, {
        key: () => ["page", params.topic, params.page]
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