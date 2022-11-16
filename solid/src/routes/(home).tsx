import { Topic, User } from "@prisma/client";
import { For, Component } from "solid-js";
import { A, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";
import { prisma } from "~/server/db/client"

export function routeData() {
    return createServerData$(async (_, { request }) => {
        const topics = await prisma.topic.findMany({
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

const TopicButton: Component<TopicProps> = (props) => {
    return (
        <div
            classList={{
                "border border-sky-500": true,
                "my-3": props.margin,
                "my-0": !props.margin,
            }}
        >
            <A href={encodeURIComponent(props.topic.title)}>
                <p>{props.topic.title}</p>
                <p>Avtor:</p>
                <For each={props.topic.authors}>{(author, i) =>
                    <p>{author.name}</p>
                }
                </For>
            </A>
        </div>
    )
}

export default Home