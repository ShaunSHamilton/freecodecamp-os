import {
  createRootRoute,
  createRoute,
  createRouter,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { Landing, landingLoader } from "./templates/landing";
import { Header } from "./components/header";
import { ProjectLesson, projectLoader } from "./templates/project";

function RootComponent() {
  return (
    <>
      <Header />
      <Outlet />
    </>
  );
}

const rootRoute = createRootRoute({
  component: RootComponent,
  notFoundComponent: () => {
    return (
      <div>
        <h2>404</h2>
        <Link to="/">Home</Link>
      </div>
    );
  },
});

const landingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: Landing,
  loader: landingLoader,
});

const lessonRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/project/$project_id/$lesson_id",
  component: ProjectLesson,
  loader: projectLoader,
});

const routeTree = rootRoute.addChildren([landingRoute, lessonRoute]);

// Set up a Router instance
export const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  defaultStaleTime: 5000,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
