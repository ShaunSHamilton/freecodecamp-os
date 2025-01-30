import { getRouteApi } from "@tanstack/react-router";
import { Selection } from "../components/selection";
// import { Events, FreeCodeCampConf, ProjectI } from "../types";
import "./landing.css";
import { FreeCodeCampConf, Locale, Project } from "../types.ts";

// interface LandingProps {
//   sock: (type: Events, data: unknown) => void;
//   projects: ProjectI[];
//   freeCodeCampConfig: FreeCodeCampConf;
//   locale: string;
// }

export const Landing = () => {
  // export const Landing = ({
  //   sock,
  //   projects,
  //   freeCodeCampConfig,
  //   locale,
  // }: LandingProps) => {
  const routeApi = getRouteApi("/");
  const { config, state, projects } = routeApi.useLoaderData();

  const title = config.client.landing[state.locale].title;
  return (
    <>
      {title && <h1>{title}</h1>}
      <p className="description">
        {config.client.landing[state.locale].description}
      </p>
      <a className="faq" href={config.client.landing[state.locale].faq_link}>
        {config.client.landing[state.locale].faq_text}
      </a>
      <Selection {...{ projects }} />
    </>
  );
};

export const landingLoader = async () => {
  const projects: Project[] = await (await fetch("/projects")).json();
  const config: FreeCodeCampConf = await (await fetch("/config")).json();

  const state = {
    locale: Locale.En,
  };

  return {
    projects,
    config,
    state,
  };
};
