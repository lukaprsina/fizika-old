import { Navbar, ScrollArea, UnstyledButton } from "@mantine/core";
import { type NextPage } from "next";
import { useRouter } from "next/router";
import { trpc } from "../../utils/trpc";

const Home: NextPage = () => {
    const router = useRouter()
    const { topic_id: topic_id_str } = router.query

    if (typeof topic_id_str !== 'string')
        return <p>Loading</p>

    const topic_id = parseInt(topic_id_str)

    const pages = trpc.fizika.get_chapters.useQuery(topic_id);
    console.log(pages.data)

    return <Navbar>
        <Navbar.Section
            grow
            component={ScrollArea}
        >
            {pages.data?.map((page, index) => {
                return (
                    <UnstyledButton
                        sx={(theme) => ({
                            display: "block"
                        })}
                        onClick={() => router.push({ pathname: `${topic_id}/${index}` })}
                        key={index}
                    >{page.title}
                    </UnstyledButton>
                )
            })}
        </Navbar.Section>
    </Navbar>
}

export default Home