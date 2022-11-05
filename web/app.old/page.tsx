/** @jsxImportSource @emotion/react */

'use client';

import { Paper } from "@mantine/core";
import { PrismaClient } from "@prisma/client";
import Link from "next/link";

async function Page() {
    const prisma = new PrismaClient();

    const courses = await prisma.course.findMany({ where: { title: "Fizika" } });
    const fizika = courses[0]
    let topics = await prisma.topic.findMany({ where: { course: fizika } });

    return <>
        <div>{topics.map(topic => (
            <Paper key={topic.id}>
                <Link key={topic.id} href={`/course/${topic.id}`}>
                    {topic.title}
                </Link>

                <p>{topic.subtitle}</p>
                <p>{topic.year}</p>
            </Paper>
        ))}</div>

    </>;
}

export default Page;