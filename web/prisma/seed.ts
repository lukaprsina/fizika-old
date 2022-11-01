import fs from 'fs';
import { PrismaClient } from "@prisma/client";
import path from 'path';

const prisma = new PrismaClient()

async function main() {
    const fizika = await prisma.course.create({
        data: {
            title: "Fizika",
            metadata: {
                create: {
                    description: "Fizika course metadata",
                    keywords: {
                        create: [
                            { text: "Elektrika" },
                            { text: "Nihanje" },
                        ]
                    }
                }
            },
            resource: {
                create: {

                }
            },
            topics: {
                create: [
                    {
                        title: "Nihanje",
                        subtitle: "Nauči se nihanje",
                        year: "2015",
                        metadata: {
                            create: {
                                description: "Nihanje description",
                            }
                        },
                        resource: {
                            create: {

                            }
                        },
                        authors: {
                            create: {
                                name: "Luka P"
                            }
                        },
                        pages: {
                            create: [
                                {
                                    html: "<div>a</div>",
                                    text: "a",
                                    metadata: {
                                        create: {
                                            description: "Page description"
                                        }
                                    },
                                    resource: {
                                        create: {
                                            modals: {
                                                create: [
                                                    {
                                                        html: "<div>modal</div>",
                                                        text: "modal",
                                                        heading: "Modal on page"
                                                    }
                                                ]
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        }
    })

    const courses_dir = "../rust/courses_output";
    let i = 0;
    while (true) {
        if (i == 2 || i == 3) {
            i++;
            continue;
        }

        const course_dir = path.join(courses_dir, i.toString());
        let j = 0;
        if (fs.existsSync(course_dir)) {
            while (true) {

            }
        }
    }
}

main()
    .then(async () => {
        await prisma.$disconnect()
    })
    .catch(async (e) => {
        console.error(e)
        await prisma.$disconnect()
        process.exit(1)
    })