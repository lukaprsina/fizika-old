import { PrismaClient } from "@prisma/client";

export default async function Page() {
    const prisma = new PrismaClient();

    const courses = await prisma.course.findMany();

    console.log(courses)
    return <>
        <p>Hello, Next.js!</p>
        <div>{courses.map(course => (
            <div key={course.id}>
                <p>{course.title}</p>
            </div>
        ))}</div>

    </>;
}