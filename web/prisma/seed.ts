import fs from 'fs';
import { PrismaClient } from "@prisma/client";
import path from 'path';

const prisma = new PrismaClient()

async function main() {
    const courses_dir = "../rust/courses_output";
    let i = 0;

    const fizika_course = await prisma.course.create({
        data: {
            title: "Fizika",
            metadata: {
                create: {
                    description: "Fizika course metadata desciption"
                },
            },
            resource: {
                create: {

                }
            }
        }
    })

    while (true) {
        if (i == 2 || i == 3) {
            i++;
            continue;
        }

        const course_dir = path.join(courses_dir, i.toString());
        let j = 0;
        console.log("course", course_dir)

        if (fs.existsSync(course_dir)) {
            const topic = await prisma.topic.create({
                data: {
                    title: "Elektrika",
                    subtitle: "Elektrika subtitle",
                    year: "2. letnik",
                    authors: {
                        create: [
                            { name: "Luka Pršina" }
                        ]
                    },
                    course: { connect: { id: fizika_course.id } },
                    metadata: {
                        create: {
                            description: "Topic metadata desciption"
                        },
                    },
                    resource: {
                        create: {

                        }
                    }
                }
            })

            while (true) {
                const exercise_dir = path.join(course_dir, `pages/page_${j}`)
                const exercise_file = path.join(exercise_dir, "index.html")

                const popups_dir = path.join(exercise_dir, "popups");
                let popup_ids = []
                console.log("\tpage dir", exercise_dir)

                if (fs.existsSync(popups_dir)) {
                    const contents = fs.readdirSync(popups_dir)

                    for (const popup_file of contents) {
                        const popup_path = path.join(popups_dir, popup_file)
                        console.log("\t\tpopup", popup_file, popup_path)

                        const file = fs.readFileSync(popup_path);
                        const popup = await prisma.modal.create({
                            data: {
                                html: file.toString(),
                                text: "Modal text",
                                heading: "Modal heading"
                            }
                        });

                        popup_ids.push({ id: popup.id })
                    }
                }

                if (fs.existsSync(exercise_file)) {
                    await prisma.page.create({
                        data: {
                            html: fs.readFileSync(exercise_file).toString(),
                            text: "Page text",
                            title: "Page title",
                            topic: { connect: { id: topic.id } },
                            resource: {
                                create: {
                                    modals: {
                                        connect: popup_ids
                                    }
                                }
                            },
                            metadata: {
                                create: {
                                    description: "Page description",
                                    keywords: {
                                        create: [
                                            { text: "Page keyword" }
                                        ]
                                    }
                                }
                            }
                        }
                    })
                } else {
                    break;
                }

                j += 1;
            }
        } else {
            break;
        }

        i += 1;
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

/* const fizika = await prisma.course.create({
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
}) */