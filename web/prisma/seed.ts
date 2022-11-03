import fs from 'fs';
import { PrismaClient } from "@prisma/client";
import path from 'path';
import { assert } from 'console';

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

        const config_path = path.join(course_dir, "config.json")
        const config_file = fs.readFileSync(config_path).toString();
        const config_json = JSON.parse(config_file);

        const script_path = path.join(course_dir, "script.json")
        const script_file = fs.readFileSync(script_path).toString();
        const script_json = JSON.parse(script_file);

        const script_title = script_json.metadata.title.substring(3)
        assert(config_json.heading == script_title)
        assert(config_json.goals == script_json.metadata.goals)

        const authors = script_json.metadata.author.map((author: string) => ({
            create: { name: author },
            where: { name: author }
        }))

        const keywords = script_json.metadata.keyword.map((keyword: string) => ({
            create: { value: keyword },
            where: { value: keyword }
        }))

        if (fs.existsSync(course_dir)) {
            const topic = await prisma.topic.create({
                data: {
                    id: i,
                    title: config_json.heading,
                    year: config_json.year,
                    authors: {
                        connectOrCreate: authors
                    },
                    course: { connect: { id: fizika_course.id } },
                    metadata: {
                        create: {
                            description: script_json.metadata.description,
                            goals: script_json.metadata.goals,
                            license: script_json.metadata.license,
                            keywords: {
                                connectOrCreate: keywords
                            }
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

                const page_path = path.join(exercise_dir, "config.json")
                const page_file = fs.readFileSync(page_path).toString();
                const page_json = JSON.parse(page_file);

                if (fs.existsSync(exercise_file)) {
                    await prisma.page.create({
                        data: {
                            id: j,
                            html: fs.readFileSync(exercise_file).toString(),
                            text: "Page text",
                            title: page_json.subheading,
                            topic: { connect: { id: topic.id } },
                            resource: {
                                create: {
                                    modals: {
                                        connect: popup_ids
                                    }
                                }
                            },
                            metadata: { create: {} }
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