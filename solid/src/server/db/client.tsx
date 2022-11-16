import { PrismaClient } from "@prisma/client";

import { env } from "~/env/server.mjs";

// eslint-disable-next-line no-var

declare global {
    var prisma: PrismaClient | undefined;
}

export const prisma =
    globalThis.prisma ||
    new PrismaClient({
        log:
            env.NODE_ENV === "development" ? ["query", "error", "warn"] : ["error"],
    });

if (env.NODE_ENV !== "production") {
    globalThis.prisma = prisma;
}
