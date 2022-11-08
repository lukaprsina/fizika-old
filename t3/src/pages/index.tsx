import { type NextPage } from "next";
import React from "react";
import { trpc } from "../utils/trpc";

const Home: NextPage = () => {
    const topics = trpc.fizika.get_topics.useQuery("Fizika");

    return <div>
        {topics.data ? (
            <section
                className="w-full px-5 bg-accent"
            >{
                    topics.data.map((topic, index) => {
                        return (
                            <div
                                className="w-full my-2 bg-white rounded-sm px-2 shadow-lg"
                                key={index}
                            >
                                <p>{topic.title}</p>
                                {topic.authors.map(author => (
                                    <p key={author.id}>{author.name}</p>
                                ))}
                            </div>
                        )
                    })
                }</section>
        ) : <p>Loading ...</p>}
    </div>
}

export default Home;