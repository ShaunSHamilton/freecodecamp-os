import { Project } from "../types";
import { Block } from "./block";

export interface SelectionProps {
  projects: Project[];
}
export const Selection = ({ projects }: SelectionProps) => {
  return (
    <ul className="blocks">
      {projects
        .sort((a, b) => a.meta.id - b.meta.id)
        .map((p) => {
          return <Block key={p.meta.id} {...{ ...p }} />;
        })}
    </ul>
  );
};
