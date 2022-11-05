import { Button } from '@mantine/core';
import { Page } from '@prisma/client';
import { GetServerSideProps, NextPage } from 'next'
import { useRouter } from 'next/router';
import { prisma } from '../../../../components/Prisma'

export const getServerSideProps: GetServerSideProps = async (context) => {
    const { topic_id, page_id } = context.query;
    if (typeof topic_id !== 'string' || typeof page_id !== 'string') {
        return {
            props: {}
        }
    }

    const t_id = parseInt(topic_id);
    const p_id = parseInt(page_id);
    if (isNaN(t_id) || isNaN(p_id)) {
        return {
            props: {}
        }
    }

    const page = await prisma.page.findFirstOrThrow({ where: { id: p_id } });

    return {
        props: {
            page
        }
    }
}

type PageProps = {
    page?: Page
}

const Page: NextPage<PageProps> = ({ page }) => {
    const router = useRouter()

    if (!page) {
        console.log("Not ready")
        return <p>Not ready</p>
    }

    if (typeof router.query.page_id !== 'string')
        return <p>Not ready</p>

    let page_id = parseInt(router.query.page_id);

    if (isNaN(page_id))
        return <p>Not ready</p>

    const can_next = true;
    const can_prev = page_id > 0;

    return <>
        {can_prev ? (
            <Button
                onClick={() => {
                    router.push(`/course/${router.query.topic_id}/page/${page_id - 1}`)
                }}>
                Previous
            </Button>
        ) : null}
        {can_next ? (
            <Button
                onClick={() => {
                    router.push(`/course/${router.query.topic_id}/page/${page_id + 1}`)
                }}
            >
                Next
            </Button>
        ) : null}
        <div
            dangerouslySetInnerHTML={{ __html: page.html }}
        ></div>
    </>;
}

export default Page
