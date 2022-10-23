import { Accordion, ActionIcon, AppShell, Header, Navbar } from "@mantine/core";
import { NextPage } from "next";
import Link from "next/link";
import useSWR, { Fetcher } from "swr";
import type { Data } from "./api/list_courses";
import { IconPlayerPlay } from "@tabler/icons"

const fetcher: Fetcher<Data, string> = async (...args) => {
    const res = await fetch(...args);
    return await res.json();
}

const Homepage: NextPage = () => {
    return (
        <AppShell
            padding="md"
            navbar={<Navbar width={{ base: 300 }} height={500} p="xs">{/* Navbar content */}</Navbar>}
            header={<Header height={60} p="xs">{/* Header content */}</Header>}
        >
            <CoursesMenu />
        </AppShell>
    )
}

function CoursesMenu() {
    const { data: courses } = useSWR(`/api/list_courses`, fetcher)

    return <Accordion defaultValue="">
        {courses?.map((chapter_info, index) => (
            <Accordion.Item value={chapter_info.heading} key={chapter_info.heading}>
                <Accordion.Control>
                    <h4>{chapter_info.heading}</h4>
                </Accordion.Control>
                <Accordion.Panel>
                    <h5>{chapter_info.author}</h5>
                    <p>{chapter_info.goals}</p>
                    <Link
                        href={{
                            pathname: '/course/[id]',
                            query: { id: index }
                        }}
                        passHref>
                        <ActionIcon component="a">
                            <IconPlayerPlay />
                        </ActionIcon>
                    </Link>
                </Accordion.Panel>
            </Accordion.Item>
        ))}
    </Accordion>
}

export default Homepage;