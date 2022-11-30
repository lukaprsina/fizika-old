/* eslint-disable @typescript-eslint/consistent-type-imports */
import { createShortcut } from "@solid-primitives/keyboard";
import {
    Button, Tab,
    TabGroup,
    TabList,
    TabPanel
} from 'solid-headless';
import { AiOutlineArrowLeft, AiOutlineArrowRight } from 'solid-icons/ai';
import { Component, createEffect, createSignal, For, JSX, Match, ParentComponent, Show, Switch } from "solid-js";
import type { RouteDataArgs } from "solid-start";
import { A, useNavigate, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Footer from "~/components/layout/Footer";
import Header from '~/components/layout/Header';
import TinyMCE from "~/components/TinyMCE";
import { AppShellContent, AppShellFooter, AppShellHeader, useEditToggle } from "~/layouts/Providers";
import { authenticator } from "~/server/auth";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    return createServerData$(async ([_, topicArg, pageArg], { request }) => {
        const topic = await prisma.topic.findUnique({
            where: {
                title: topicArg
            }
        });

        const page_id = parseInt(pageArg);

        if (isNaN(page_id)) return null;
        if (!topic) return null;

        const page = await prisma.page.findUnique({
            where: {
                topicId_id: {
                    id: page_id,
                    topicId: topic.id,
                }
            }
        });

        const page_count = await prisma.page.count({
            where: {
                topicId: topic.id,
            }
        })

        const user = await authenticator.isAuthenticated(request);

        return { page, user, page_count };
    }, {
        key: () => ["page", params.topic, params.page]
    })
}

type ParamsType = {
    topic: string;
    page: string;
}

type TabType = {
    name?: string;
    content?: JSX.Element;
}

const PageNavbar: Component = () => {
    const page_data = useRouteData<typeof routeData>();
    const params = useParams<ParamsType>();
    const editToggle = useEditToggle();
    const [activeTab, setActiveTab] = createSignal()
    const [showEditor, setShowEditor] = createSignal(false);

    createEffect(() => {
        if (page_data()?.user && editToggle?.edit()) {
            console.log("Show editor")
            setShowEditor(true);
        } else {
            setShowEditor(false);
        }
    })

    const tabs = [
        {
            name: "Navbar",
        },
        {
            name: "Page",
            content: (
                <Show when={page_data()}>
                    <div>
                        <Switch
                            fallback={
                                // eslint-disable-next-line solid/no-innerhtml
                                <div innerHTML={page_data()?.page?.html ?? ""} />
                            }
                        >
                            <Match when={showEditor()}>
                                <TinyMCE show={showEditor()} />
                            </Match>
                        </Switch>
                        <NavButtons page_count={page_data()?.page_count ?? 0} />
                    </div>
                </Show>
            )
        },
        {
            name: "Explanation"
        }
    ];

    createEffect(() => console.log(activeTab()))

    return <>
        <div class="bottom-0 bg-inherit text-white left-0 right-0 fixed z-40 flex justify-around">
            <For each={tabs}>{(tab, i) => (
                <Button onClick={() => setActiveTab(i)}>{tab.name}</Button>
            )}</For>
        </div>
        <AppShellHeader>
            <Header topic={params.topic} user={page_data()?.user} />
        </AppShellHeader>
        <AppShellContent>
            {tabs[1].content}
        </AppShellContent>
    </>

    /* return (
        <TabGroup
            horizontal={true}
            defaultValue="Page"
            class="min-h-screen flex flex-col bg-inherit">
            {({ isSelected }) => <>
                <AppShellHeader>
                    <Header topic={params.topic} user={page_data()?.user} />
                </AppShellHeader>
                <AppShellContent>
                    <div class="flex-grow bg-inherit">
                        <For each={tabs()}>{(tab) => (
                            <TabPanel value={tab.name} class="bg-inherit">
                                {tab.content ?? tab.name}
                            </TabPanel>
                        )}
                        </For>
                    </div>
                </AppShellContent>
                <TabList class="flex flex-1 flex-wrap justify-center w-full border-b-2 border-slate-300 dark:border-slate-700 box-border h-9">
                    <For each={tabs()}>{(tab) => (
                        <Tab
                            class="flex sticky flex-grow mb-[-2px] hover:bg-slate-50 dark:hover:bg-slate-800 items-center justify-center rounded-t-md z-0 box-border border-slate-300 border-b-2 hover:cursor-pointer"
                            classList={{
                                "border-sky-500": isSelected(tab.name ?? "Page")
                            }}
                            value={tab.name}
                        >{tab.name}</Tab>
                    )}</For>
                </TabList>
                <AppShellFooter>
                    <Footer />
                </AppShellFooter>
            </>}
        </TabGroup>
    ) */
}

const NavButtons: Component<{ page_count: number }> = (props) => {
    const params = useParams<ParamsType>();
    const [pageId, setPageId] = createSignal(NaN);
    const [baseURL, setBaseURL] = createSignal("");
    const icon_size = "25px";
    const navigate = useNavigate();

    createEffect(() => {
        setBaseURL("/" + params.topic + "/")
        setPageId(parseInt(params.page));
    })

    createShortcut(
        ["ARROWLEFT"],
        () => {
            if (pageId() > 0) {
                navigate(baseURL() + (pageId() - 1))
            }
        },
        { preventDefault: true, requireReset: false }
    )

    createShortcut(
        ["ARROWRIGHT"],
        () => {
            if (pageId() < props.page_count - 1) {
                navigate(baseURL() + (pageId() + 1))
            }
        },
        { preventDefault: true, requireReset: false }
    )

    return (
        <div
            class="w-full flex my-10 justify-items-end"
            classList={{
                "justify-end": pageId() <= 0,
                "justify-around": pageId() > 0,
            }}
        >
            <Show when={pageId() > 0}>
                <IconButton>
                    <A href={baseURL() + (pageId() - 1)}>
                        <AiOutlineArrowLeft size={icon_size} />
                    </A>
                </IconButton>
            </Show>
            <Show when={pageId() < props.page_count - 1}>
                <IconButton>
                    <A href={baseURL() + (pageId() + 1)}>
                        <AiOutlineArrowRight size={icon_size} />
                    </A>
                </IconButton>
            </Show>
        </div>
    )
}

const IconButton: ParentComponent = (props) => {
    return (
        <Button
            class="text-slate-600 hover:bg-slate-50 rounded-md"
        >
            {props.children}
        </Button>
    )
}

export default PageNavbar