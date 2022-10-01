import { Button } from "@mantine/core";
import { NextPage, NextApiHandler } from "next";

const Database: NextPage = () => {
    const callAPI = async () => {
        try {
            const res = await fetch(`/api/list_courses`);
            const data = await res.json();
            console.log(data);
        } catch (err) {
            console.log(err);
        }
    };

    return (
        <Button
            onClick={() => {
                callAPI()
            }}
        >Database</Button>
    )
}

export default Database;