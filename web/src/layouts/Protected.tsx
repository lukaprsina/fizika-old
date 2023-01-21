import { createEffect, Match, Switch, type Component } from "solid-js";
import { RouteDataArgs, useRouteData } from "solid-start";
import { createServerData$, redirect } from "solid-start/server";
import { authenticator } from "~/server/auth";
import { type User } from "@prisma/client";
import Page, { routeData } from "~/routes/(home)"

export const withProtected = (Component: ProtectedRouter) => {
  const routeData = () => {
    return createServerData$(async (_, { request }) => {
      const user = await authenticator.isAuthenticated(request);
      if (!user) {
        console.log("Not logged in")
        throw redirect("/failure");
      } else {
        console.log("Logged in", user.displayName)
      }
      console.log("From server", { user })
      return user;
    }, {
      key: ["test"]
    });
  };
  return {
    routeData,
    Page: () => {
      const current = useRouteData<typeof routeData>();
      createEffect(() => console.log(current()))
      return (
        <Switch fallback={<Component {...(current() as User)} />}>
          <Match when={current.loading || current() instanceof Response}>
            <h1>Nalagam...</h1>
          </Match>
        </Switch>
      );
    },
  };
};

export type ProtectedRouter = Component<User>;
