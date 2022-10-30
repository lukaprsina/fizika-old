import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient()

async function main() {
    const fizika = await prisma.course.create({
        data: {
            title: "Fizika",
            topics: {
                create: {
                    year: "2",
                    title: "Elektrika",
                    subtitle: "LALALA",
                    authors: {
                        create: [
                            {
                                name: "Luka Prši"
                            }
                        ]
                    },
                    pages: {
                        create: {
                            html: "<div>a</div>",
                            text: "a",
                            modals: {
                                create: [
                                    {
                                        heading: "Modal heading",
                                        html: "<brug></brug>",
                                        text: "scanje",
                                    }
                                ]
                            }
                        }
                    }
                }
            },
            modals: {
                create: {
                    heading: "\"Global modal\" heading",
                    html: "<brug></brug>",
                    text: "scanje",
                }
            },
        }
    });

    console.log(fizika)
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