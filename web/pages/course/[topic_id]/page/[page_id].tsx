import { Paper } from '@mantine/core';
import { Page } from '@prisma/client';
import { GetServerSideProps, NextPage } from 'next'
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
    if (!page) {
        console.log("Not ready")
        return <p>Not ready</p>
    }

    return <>
        <div
            dangerouslySetInnerHTML={{ __html: page.html }}
        ></div>
    </>;
}

export default Page
