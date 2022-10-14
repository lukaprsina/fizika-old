import { NextPage } from "next"
import { Accordion, AppShell, Header, Navbar, Paper, Skeleton } from "@mantine/core";
import { useRouter } from "next/router"
import { Data } from "../../api/list_chapters"
import { server } from '../../../config';
import useSWR, { Fetcher } from "swr";
import NoSSR from "../../../components/NoSSR";
import { Suspense } from "react";
import TinyMCE from "../../../components/TinyMCE";

const fetcher: Fetcher<Data, string> = async (...args) => {
    const res = await fetch(...args);
    return await res.json();
}

const Course: NextPage = () => {
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
                    <CoursesMenu />
                </ Suspense>
            }</Navbar>}
            header={<Header height={60} p="xs">{/* Header content */}</Header>}
        >
            <NoSSR>
                <TinyMCE />
            </NoSSR>
        </AppShell>
    )
}


function CoursesMenu() {
    const router = useRouter()
    let url = router.query.id ? `${server}/api/list_chapters/?course=${router.query.id}` : null
    const { data: chapters } = useSWR(url, fetcher, { suspense: true })

    return <Accordion defaultValue="">
        {chapters?.map((course_info, i) => (
            <Paper
                key={i}
                shadow="xs"
                p="md"
            >{course_info.subheading}</Paper>
        ))}
    </Accordion>
}

export default Course