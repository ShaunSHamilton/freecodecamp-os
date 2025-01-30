import { useQuery } from "@tanstack/react-query";

import { FreeCodeCampConf, Locale, Project } from "../types.ts";
import { Selection } from "../components/selection";
import "./landing.css";

export const Landing = () => {
  const { isPending, isError, data, error } = useQuery({
    queryKey: ["landing"],
    queryFn: landingLoader,
  });

  if (isPending) {
    return <h1>LOading</h1>;
  }

  if (isError) {
    return <h1>{error.message}</h1>;
  }

  const { config, state, projects } = data;

  return (
    <>
      {config.client.landing[state.locale].title && (
        <h1>{config.client.landing[state.locale].title}</h1>
      )}
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
