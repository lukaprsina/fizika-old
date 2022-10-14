import { NextPage } from "next"
import { AppShell, Box, Button, Header, Navbar, Paper, Skeleton, Text } from "@mantine/core";
import { useRouter } from "next/router"
import { Data as ChapterData } from "../../api/list_chapters"
import { Data as HtmlData } from "../../api/get_html"
import useSWR, { Fetcher } from "swr";
import NoSSR from "../../../components/NoSSR";
import { Suspense, useState } from "react";
import TinyMCE from "../../../components/TinyMCE";

const course_fetcher: Fetcher<ChapterData, string> = async (...args) => {
    const res = await fetch(...args);
    return await res.json();
}

const html_fetcher: Fetcher<HtmlData, string> = async (...args) => {
    const res = await fetch(...args);
    return await res.json();
}

const Course: NextPage = () => {
    const router = useRouter()
    const [page, setPage] = useState(0)
    const url = router.query.id ? `/api/list_chapters/?course=${router.query.id}` : null
    const { data: chapters } = useSWR(url, course_fetcher)

    const url2 = router.query.id ? `/api/get_html/?course=${router.query.id}&page=${page}` : null
    const { data: html } = useSWR(url2, html_fetcher)

    /* const getHtml: () => HtmlData | undefined = () => {
        return html
    } */

    return (
        <AppShell
            padding="md"
            navbar={<Navbar width={{ base: 300 }} height={500} p="xs">{
                <Suspense fallback={
                    <>
                        {Array.apply(null, Array(5)).map((x, i) =>
                            <Skeleton sx={{
                                margin: "25px 0"
                            }} height={100} radius="xl" key={i} />
                        )}

                    </>
                }>
                    <CoursesMenu setPage={setPage} chapters={chapters} />
                </ Suspense>
            }</Navbar>}
            header={<Header height={60} p="xs">{/* Header content */}</Header>}
        >
            <NoSSR>
                <TinyMCE html={html?.file} />
            </NoSSR>
        </AppShell>
    )
}

type CoursesMenuType = {
    setPage: any;
    chapters: ChapterData | undefined
}

function CoursesMenu({ setPage, chapters }: CoursesMenuType) {
    return <Box
        sx={{
            overflow: "auto"
        }}
    >
        {chapters?.map((course_info, index) => (
            <Paper
                key={index}
                shadow="xs"
                p="md"
            >
                <Text variant="link" component="a" onClick={() => {
                    console.log("Page: ", index)
                    setPage(index)
                }}>
                    {course_info.subheading}
                </Text>
            </Paper>
        ))
        }
    </Box >
}

export default Course