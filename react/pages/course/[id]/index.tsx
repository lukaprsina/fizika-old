import { NextPage } from "next"
import { useRouter } from "next/router"

const Course: NextPage = () => {
    const router = useRouter()

    return (
        <p>Id: {router.query.id}</p>
    )
}

export default Course