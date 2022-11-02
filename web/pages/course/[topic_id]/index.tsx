import { Paper } from '@mantine/core';
import { Page } from '@prisma/client';
import { GetServerSideProps, NextPage } from 'next'
import Link from 'next/link';
import { useRouter } from 'next/router'
import { prisma } from '../../../components/Prisma'

export const getServerSideProps: GetServerSideProps = async (context) => {
    const { topic_id } = context.query;
    if (typeof topic_id !== 'string') {
        return {
            props: {}
        }
    }

    const id = parseInt(topic_id);
    if (isNaN(id)) {
        return {
            props: {}
        }
    }

    const topic = await prisma.topic.findFirstOrThrow({ where: { id } });
    let pages = await prisma.page.findMany({ where: { topic } });

    return {
        props: {
            pages
        }
    }
}

type TopicProps = {
    pages: Page[]
}

const Topic: NextPage<TopicProps> = ({ pages }) => {
    const router = useRouter()
    console.log(router.query)

    return (
        <div>{pages.map(page => (
            <Paper
                key={page.id}
                shadow="xs"
            >
                <Link key={page.id} href={`/course/${router.query.topic_id}/page/${page.id}`}>
                    {page.title}
                </Link>
            </Paper>
        ))}</div>
    );
}

export default Topic
