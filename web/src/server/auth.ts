import { type User } from "@prisma/client";
import { Authenticator } from "@solid-auth/core";
import { GitHubStrategy, GoogleStrategy, MicrosoftStrategy } from "@solid-auth/socials";
import { serverEnv } from "~/env/server";
import { sessionStorage } from "~/utils/auth";
import { prisma } from "./db/client";

export const authenticator = new Authenticator<User>(sessionStorage).use(new GitHubStrategy(
  {
    clientID: serverEnv.CLIENT_ID_GITHUB,
    clientSecret: serverEnv.CLIENT_SECRET_GITHUB,
    callbackURL: serverEnv.SITE_URL + "/api/auth/github/callback",
    scope: [],
  },
  async ({ profile }) => {
    let user = await prisma.user.findUnique({
      where: {
        id: profile.id,
      },
    });

    console.log("GITHUB:", profile)

    if (!user) {
      user = await prisma.user.create({
        data: {
          id: profile.id,
          displayName: profile.displayName,
        },
      });
    }

    return user;
  }
)).use(new MicrosoftStrategy(
  {
    clientID: serverEnv.CLIENT_ID_MICROSOFT,
    clientSecret: serverEnv.CLIENT_SECRET_MICROSOFT,
    callbackURL: serverEnv.SITE_URL + "/api/auth/microsoft/callback",
    scope: ["openid"],
  },
  async ({ profile }) => {
    let user = await prisma.user.findUnique({
      where: {
        id: profile.id,
      },
    });

    console.log("MICROSOFT:", profile)

    if (!user) {
      user = await prisma.user.create({
        data: {
          id: profile.id,
          displayName: profile.displayName,
        },
      });
    }

    return user;
  }
)).use(new GoogleStrategy(
  {
    clientID: serverEnv.CLIENT_ID_GOOGLE,
    clientSecret: serverEnv.CLIENT_SECRET_GOOGLE,
    callbackURL: serverEnv.SITE_URL + "/api/auth/google/callback",
    scope: ["openid"],
  },
  async ({ profile }) => {
    let user = await prisma.user.findUnique({
      where: {
        id: profile.id,
      },
    });

    console.log("GOOGLE:", profile)

    if (!user) {
      user = await prisma.user.create({
        data: {
          id: profile.id,
          displayName: profile.displayName,
        },
      });
    }

    return user;
  }
))