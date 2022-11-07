import { z } from "zod";

import { router, publicProcedure } from "../trpc";

export const fizikaRouter = router({
    get_topics: publicProcedure.input(z.string()).query(({ ctx, input }) => {
        return ctx.prisma.topic.findMany({
            where: {
                course: { title: input }
            }
        });
    })
});
