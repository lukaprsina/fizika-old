import { createShortcut } from "@solid-primitives/keyboard";
import {
    Button
} from 'solid-headless';
import { AiOutlineArrowLeft, AiOutlineArrowRight } from 'solid-icons/ai';
import type { Accessor, Component, JSX, ParentComponent, Setter } from "solid-js";
import { createEffect, createSignal, Show } from "solid-js";
import type { RouteDataArgs } from "solid-start";
import { A, useNavigate, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import { useEditToggle } from "~/layouts/Providers";
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

    /* const a = <TabGroup>{({ isActive }) => {
        return <p>Shit</p>
    }</TabGroup> */

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
                    index={1}
                    hidden={true}
                >
                    Editor
                </TabButton>
                <TabButton
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                    index={2}
                >
                    Explanation
                </TabButton>
            </TabButtonsContainer>
            <TabsContainer>
                <Tab
                ></Tab>
            </TabsContainer>
        </>}</TabsContext>
    )
}

type TabsContextType = {
    defaultIndex?: number;
    children?: (properties: {
        activeTab: Accessor<number>,
        setActiveTab: Setter<number>,
    }) => JSX.Element;
}

const TabsContext: Component<TabsContextType> = (props) => {
    const [activeTab, setActiveTab] = createSignal(props.defaultIndex ?? 0);
    const [children, setChildren] = createSignal<JSX.Element>();

    createEffect(() => {
        if (props.children) {
            setChildren(props.children({
                activeTab,
                setActiveTab,
            }))
        }
    })

    return <div>
        {children()}
    </div>
}

type TabButtonsContainerType = {
    defaultIndex?: number;
}

const TabButtonsContainer: ParentComponent<TabButtonsContainerType> = (props) => {
    return (
        <div class="bottom-0 bg-inherit text-white left-0 right-0 fixed z-40 flex justify-around">
            {props.children}
        </div>
    )
}

type TabButtonType = {
    index: number;
    setActiveTab: Setter<number>;
    activeTab: Accessor<number>;
    hidden?: boolean;
}

const TabButton: ParentComponent<TabButtonType> = (props) => {
    return (
        <Button
            onClick={() => props.setActiveTab(props.index)}
            class="flex sticky flex-grow mb-[-2px] hover:bg-slate-50 dark:hover:bg-slate-800 items-center justify-center rounded-t-md z-0 box-border border-slate-300 border-b-2 hover:cursor-pointer"
            classList={{
                "border-sky-500": props.activeTab() == props.index,
                "hidden": props.hidden
            }}
        >
            {props.children}
        </Button>
    )
}

type TabsContainerType = {
};

const TabsContainer: ParentComponent<TabsContainerType> = (props) => {
    return (
        <>
            {props.children}
        </>
    )
}

type TabType = {
};

const Tab: ParentComponent<TabType> = (props) => {
    return (
        <>
            {props.children}
        </>
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