import { z, ZodNumber } from "zod";

import { router, publicProcedure } from "../trpc";

export const fizikaRouter = router({
    get_topics: publicProcedure.input(z.string()).query(({ ctx, input }) => {
        return ctx.prisma.topic.findMany({
            where: {
                course: { title: input }
            },
            include: {
                authors: {}
            },
            orderBy: { year: "asc" },
        });
    }),
    get_chapters: publicProcedure.input(z.number()).query(({ ctx, input }) => {
        return ctx.prisma.page.findMany({
            where: {
                topicId: input
            },
            select: {
                title: true,
                metadataId: true,
                resourceId: true
            }
        })
    }),
    get_page: publicProcedure.input(z.object({ topic_id: z.number(), page_id: z.number() })).query(({ ctx, input }) => {
        return ctx.prisma.page.findFirst({
            select: {
                title: true,
                metadataId: true,
                resourceId: true,
                html: true,
            },
            where: {
                topicId: input.topic_id,
                id: input.page_id
            }
        })
    })
});
