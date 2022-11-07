import { type NextPage } from "next";
import React from "react";
import { trpc } from "../utils/trpc";

const Home: NextPage = () => {
    const topics = trpc.fizika.get_topics.useQuery("Fizika");

    return <div>
        {topics.data ? <section>{
            topics.data.map((topic, index) => <div key={index}>
                <p>{topic.title}</p>
            </div>)
        }</section> : <p>Loading ...</p>}
    </div>
}

export default Home;