import { PrismaClient, Topic, User } from "@prisma/client";
import { For, Component } from "solid-js";
import { A, useRouteData } from "solid-start"
import { createServerData$ } from "solid-start/server";

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
        <main class="">
            <For each={topics()}>{(topic, i) =>
                <TopicButton topic={topic} />
            }
            </For>
        </main>
    )
}

type TopicWithAuthor = Topic & {
    authors: User[];
};

type TopicProps = {
    topic: TopicWithAuthor
}

const TopicButton: Component<TopicProps> = ({ topic }) => {
    return (
        <A
            href={topic.id.toString()}
            class="bg-black text-white"
        >
            <p>{topic.title}</p>
            <p>Avtor:</p>
            <For each={topic.authors}>{(author, i) =>
                <p>{author.name}</p>
            }
            </For>
        </A>
    )
}

export default Home