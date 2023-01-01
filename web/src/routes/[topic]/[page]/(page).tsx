import { createShortcut } from "@solid-primitives/keyboard";
import { Button } from "solid-headless";
import { AiOutlineArrowLeft, AiOutlineArrowRight } from 'solid-icons/ai';
import type { Component, ParentComponent } from "solid-js";
import { createEffect, createSignal, Show } from "solid-js";
import { Link, RouteDataArgs, Style } from "solid-start";
import { A, useNavigate, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Header from "~/components/Header";
import { Tab, TabButton, TabButtonsContainer, TabsContext } from "~/components/Tabs";
import TinyMCE from "~/components/TinyMCE";
import { AppShellContent, AppShellHeader, useEditToggle } from "~/layouts/Providers";
import { authenticator } from "~/server/auth";
import { prisma } from "~/server/db/client";
import styles from "./page.module.scss"

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

    console.log({ styles })

    return <>
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
                            class={styles.page_content}
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
    </>
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