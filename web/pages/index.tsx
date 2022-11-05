import { Paper } from "@mantine/core";
import { Topic } from "@prisma/client";
import { GetServerSideProps, NextPage } from "next";
import Link from "next/link";
import { prisma } from '../components/Prisma'

export const getServerSideProps: GetServerSideProps = async (context) => {
    const fizika = await prisma.course.findFirstOrThrow({ where: { title: "Fizika" } });
    let topics = await prisma.topic.findMany({ where: { course: fizika } });

    return {
        props: {
            topics
        }
    }
}

type HomeProps = {
    topics: Topic[]
}

const Home: NextPage<HomeProps> = ({ topics }) => {
    return (
        <div>{topics.map(topic => (
            <Paper
                key={topic.id}
                shadow="xs"
            >
                <Link key={topic.id} href={`/course/${topic.id}`}>
                    {topic.title}
                </Link>

                <p>{topic.subtitle}</p>
                <p>{topic.year}</p>
            </Paper>
        ))}</div>
    );
}

export default Home;