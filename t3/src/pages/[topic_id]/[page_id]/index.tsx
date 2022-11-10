import { type NextPage } from "next";
import { useRouter } from "next/router";
import { trpc } from "../../../utils/trpc";

const Home: NextPage = () => {
    const router = useRouter()
    const { topic_id: topic_id_str, page_id: page_id_str, } = router.query

    if (typeof topic_id_str !== 'string' || typeof page_id_str !== 'string')
        return <p>Loading...</p>

    const topic_id = parseInt(topic_id_str)
    const page_id = parseInt(page_id_str)

    const page = trpc.fizika.get_page.useQuery({ topic_id, page_id });
    console.log(page.data?.title)

    return <div>
        {page.data ? (
            <div dangerouslySetInnerHTML={{ __html: page.data.html }}>
            </div>
        ) : <p>Loading...</p>}
    </div>
}

export default Home