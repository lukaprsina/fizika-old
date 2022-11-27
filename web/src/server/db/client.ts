import { PrismaClient } from "@prisma/client";
import type { TinyMCE } from "tinymce";
import { serverEnv } from "~/env/server";

declare global {
  // eslint-disable-next-line no-var
  var prisma: PrismaClient | undefined;
  // eslint-disable-next-line no-var
  var tinymce: TinyMCE;
}

export const prisma =
  global.prisma ||
  new PrismaClient({
    log:
      serverEnv.NODE_ENV === "development"
        ? ["error", "warn"]
        : ["error"],
  });

export const tinymce = global.tinymce

if (serverEnv.NODE_ENV !== "production") {
  global.prisma = prisma;
}
