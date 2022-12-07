import { createShortcut } from "@solid-primitives/keyboard";
import { Button } from "solid-headless";
import { AiOutlineArrowLeft, AiOutlineArrowRight } from 'solid-icons/ai';
import type { Component, ParentComponent } from "solid-js";
import { createEffect, createSignal, Show } from "solid-js";
import type { RouteDataArgs } from "solid-start";
import { A, useNavigate, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Header from "~/components/Header";
import { Tab, TabButton, TabButtonsContainer, TabsContext } from "~/components/Tabs";
import TinyMCE from "~/components/TinyMCE";
import { AppShellContent, AppShellHeader, useEditToggle } from "~/layouts/Providers";
import { authenticator } from "~/server/auth";
import { prisma } from "~/server/db/client";

export function routeData({ params }: RouteDataArgs) {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
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
/* createEffect(() => {
        const my_tabs = [
            {
                name: "Navbar",
                index: 0
            },
            {
                name: "Page",
                // eslint-disable-next-line solid/no-innerhtml
                content: <>
                    <Show when={page_data()?.page?.html}>
                        <div
                            // eslint-disable-next-line solid/no-innerhtml
                            innerHTML={page_data()?.page?.html}
                        />
                    </Show>
                    <NavButtons page_count={page_data()?.page_count ?? 0} />
                </>,
                index: 1,
                hidden_condition: showEditor()
            },
            {
                name: "Editor",
                content: <>
                    <TinyMCE
                        authorized={isAuthed()}
                        visible={showEditor() && activeTab() == 1}
                    />
                    <NavButtons page_count={page_data()?.page_count ?? 0} />
                </>,
                index: 1,
                hidden: true,
                hidden_condition: !showEditor()
            },
            {
                name: "Explanation",
                index: 2
            }
        ];

        console.log(my_tabs)

        setTabs(my_tabs);
    }); */

const PageNavbar: Component = () => {
    const page_data = useRouteData<typeof routeData>();
    const params = useParams<ParamsType>();
    const editToggle = useEditToggle();
    const [showEditor, setShowEditor] = createSignal(false);
    const [isAuthed, setIsAuthed] = createSignal(false);

    createEffect(() => {
        if (page_data()?.user) {
            setIsAuthed(true);
        } else {
            setIsAuthed(false);
        }
    })

    createEffect(() => {
        if (editToggle?.edit()) {
            setShowEditor(true)
        } else {
            setShowEditor(false)
        }
    })

    /* return <TabsContainer>
        <div class="bottom-0 bg-inherit text-white left-0 right-0 fixed z-40 flex justify-around">
            <For each={tabs()}>{(tab, i) => (
                <Button
                    onClick={() => setActiveTab(i)}
                    class="flex sticky flex-grow mb-[-2px] hover:bg-slate-50 dark:hover:bg-slate-800 items-center justify-center rounded-t-md z-0 box-border border-slate-300 border-b-2 hover:cursor-pointer"
                    classList={{
                        "border-sky-500": activeTab() == i(),
                        "hidden": tab.hidden
                    }}
                >
                    {tab.name}
                </Button>
            )}</For>
        </div>
        <AppShellHeader>
            <Header topic={params.topic} user={page_data()?.user} />
        </AppShellHeader>
        <AppShellContent>
            <div class="w-full min-h-full relative bg-inherit"> //mb-10
                <For each={tabs()}>{(tab) => {
                    return (
                        <div
                            hidden={activeTab() !== tab.index || tab.hidden_condition}
                            class="absolute w-full h-full top-0 left-0 bg-inherit"
                        >
                            {tab.content ?? tab.name}
                        </div>
                    )
                }}</For>
            </div>
        </AppShellContent>
    </TabsContainer> */
    return (
        <TabsContext defaultIndex={1}>{({ activeTab, setActiveTab }) => <>
            <TabButtonsContainer>
                <TabButton
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                    index={0}
                >
                    Navbar
                </TabButton>
                <TabButton
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                    index={1}
                >
                    Page
                </TabButton>
                <TabButton
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                    index={2}
                >
                    Explanation
                </TabButton>
            </TabButtonsContainer>
            <AppShellHeader>
                <Header topic={params.topic} user={page_data()?.user} />
            </AppShellHeader>
            <AppShellContent>
                <Tab
                    activeTab={activeTab}
                    index={0}
                >
                    Navbar
                </Tab>
                <Tab
                    activeTab={activeTab}
                    index={1}
                    hidden={showEditor()}
                >

                    <Show when={page_data()?.page?.html}>
                        <div
                            // eslint-disable-next-line solid/no-innerhtml
                            innerHTML={page_data()?.page?.html}
                        />
                    </Show>
                    <NavButtons page_count={page_data()?.page_count ?? 0} />

                </Tab>
                <Tab
                    activeTab={activeTab}
                    index={1}
                    hidden={!showEditor()}
                >
                    <TinyMCE
                        authorized={isAuthed()}
                        visible={showEditor() && activeTab() == 1}
                        content={page_data()?.page?.html}
                    />
                    <NavButtons page_count={page_data()?.page_count ?? 0} />
                </Tab>
                <Tab
                    activeTab={activeTab}
                    index={2}
                >
                    Explanation
                </Tab>
            </AppShellContent>
        </>}</TabsContext>
    )
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