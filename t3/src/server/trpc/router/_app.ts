import { router } from "../trpc";
import { authRouter } from "./auth";
import { exampleRouter } from "./example";
import { fizikaRouter } from "./fizika";

export const appRouter = router({
  example: exampleRouter,
  auth: authRouter,
  fizika: fizikaRouter
});

// export type definition of API
export type AppRouter = typeof appRouter;
