import { PrismaClient, Topic, User } from "@prisma/client";
import { For, Component } from "solid-js";
import { A, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import clsx from 'clsx';

export function routeData() {
    return createServerData$(async (_, { request }) => {
        const db = new PrismaClient();
        const topics = await db.topic.findMany({
            where: {
                course: { title: "Fizika" }
            },
            include: {
                authors: {}
            },
            orderBy: { year: "asc" },
        });

        return topics;
    })
}

const Home: Component = () => {
    const topics = useRouteData<typeof routeData>();

    return (
        <main class="w-screen">
            <div
                class="w-[800px] mx-auto"
            >
                <For each={topics()}>{(topic, i) =>
                    <TopicButton
                        margin={i() != 0 && i() != topics().length - 1}
                        topic={topic}
                    />
                }
                </For>
            </div>
        </main>
    )
}

type TopicWithAuthor = Topic & {
    authors: User[];
};

type TopicProps = {
    topic: TopicWithAuthor;
    margin: boolean;
}

const TopicButton: Component<TopicProps> = ({ topic, margin }) => {
    return (
        <div
            class={clsx("border border-sky-500", margin ? "my-3" : "my-0")}
        >
            <A href={topic.id.toString()}>
                <p>{topic.title}</p>
                <p>Avtor:</p>
                <For each={topic.authors}>{(author, i) =>
                    <p>{author.name}</p>
                }
                </For>
            </A>
        </div>
    )
}

export default Home