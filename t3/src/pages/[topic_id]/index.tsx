import { type NextPage } from "next";
import { useRouter } from "next/router";
import { trpc } from "../../utils/trpc";

const Home: NextPage = () => {
    const router = useRouter()
    const { topic_id: topic_id_str } = router.query
    if (typeof topic_id_str !== 'string')
        return <p>Loading...</p>

    const topic_id = parseInt(topic_id_str)

    const chapters = trpc.fizika.get_chapters.useQuery(topic_id);
    console.log(chapters.data)

    return <div>
        {chapters.data?.map((chapter, index) => {
            return (
                <div key={index}>{chapter.title}</div>
            )
        })}
    </div>
}

export default Home