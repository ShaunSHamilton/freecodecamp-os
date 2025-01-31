import { useQuery } from "@tanstack/react-query";
import { Block } from "./block";
import { getProjects } from "../utils/fetch";
import { F } from "../utils";

export interface SelectionProps {
  set_project_id: F<number, void>;
}

export const Selection = ({ set_project_id }: SelectionProps) => {
  const projectsQuery = useQuery({
    queryKey: ["projects"],
    queryFn: getProjects,
  });

  if (projectsQuery.isPending) {
    return <h3>Loading</h3>;
  }

  if (projectsQuery.isError) {
    return <p>{projectsQuery.error.message}</p>;
  }

  return (
    <ul className="blocks">
      {projectsQuery.data
        .sort((a, b) => a.meta.id - b.meta.id)
        .map((p) => {
          return <Block key={p.meta.id} {...{ ...p, set_project_id }} />;
        })}
    </ul>
  );
};
