import { useQuery } from "@tanstack/react-query";

import { FreeCodeCampConf, Project } from "../types.ts";
import { Selection } from "../components/selection";
import "./landing.css";
import { F } from "../utils/index.ts";
import { getConfig, getState } from "../utils/fetch.ts";

interface LandingProps {
  set_project_id: F<number, void>;
}

export function Landing({ set_project_id }: LandingProps) {
  const { isPending, isError, data, error } = useQuery({
    queryKey: ["landing"],
    queryFn: async () => {
      return {
        state: await getState(),
        config: await getConfig(),
      };
    },
  });

  if (isPending) {
    return <h1>LOading</h1>;
  }

  if (isError) {
    return <h1>{error.message}</h1>;
  }

  const { config, state } = data;

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
      <Selection {...{ set_project_id }} />
    </>
  );
}

export const landingLoader = async () => {
  const projects: Project[] = await (await fetch("/projects")).json();
  const config: FreeCodeCampConf = await (await fetch("/config")).json();

  const state = {
    locale: "en",
  };

  return {
    projects,
    config,
    state,
  };
};
