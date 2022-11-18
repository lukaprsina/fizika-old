import { Component, createEffect, Show, Suspense } from "solid-js";
import { RouteDataArgs, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Content from "~/components/Content";
import Navbar from "~/components/Navbar";
import { EditToggleProvider } from "~/routes/(home)";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topicArg, pageArg]) => {
        const topic = await prisma.topic.findUnique({
            where: {
                title: topicArg
            }
        });

        const page_id = parseInt(pageArg);

        if (typeof topic === "undefined") return null;// throw new Error("Topic doesn't exist");
        if (isNaN(page_id)) return null;// throw new Error("Page ID is NaN");

        const page = await prisma.page.findUnique({
            where: {
                topicId_id: {
                    id: page_id,
                    topicId: topic.id,
                }
            }
        });

        return page;
    }, {
        key: () => ["page", params.topic, params.page]
    })
}

const PageNavbar: Component = () => {
    const page = useRouteData<typeof routeData>();
    const params = useParams();

    return (
        <EditToggleProvider initial={false}>
            <Navbar topic={params.topic} />
            <Content>
                <Show when={page()}>
                    <div innerHTML={page().html}></div>
                </Show>
            </Content>
        </EditToggleProvider>
    )

}

export default PageNavbar