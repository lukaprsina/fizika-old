import { z } from "zod";

export const serverScheme = z.object({
  NODE_ENV: z.enum(['development', 'production', 'test']).default('development'),
  DATABASE_URL: z.string(),
  SITE_URL: z.string(),
  CLIENT_ID_GITHUB: z.string(),
  CLIENT_SECRET_GITHUB: z.string(),
  CLIENT_ID_GOOGLE: z.string(),
  CLIENT_SECRET_GOOGLE: z.string(),
  CLIENT_ID_MICROSOFT: z.string(),
  CLIENT_SECRET_MICROSOFT: z.string(),
});

export const clientScheme = z.object({
  MODE: z.enum(['development', 'production', 'test']).default('development'),
  VITE_SESSION_SECRET: z.string(),
});
