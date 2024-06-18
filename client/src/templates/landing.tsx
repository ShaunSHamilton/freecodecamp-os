import { Selection } from "../components/selection";
import { Events, FreeCodeCampConfigI, ProjectI } from "../types";
import "./landing.css";

interface LandingProps {
  sock: (type: Events, data: unknown) => void;
  projects: ProjectI[];
  freeCodeCampConfig: FreeCodeCampConfigI;
  locale: string;
}

export const Landing = ({
  sock,
  projects,
  freeCodeCampConfig,
  locale,
}: LandingProps) => {
  // @ts-expect-error TODO
  const title = freeCodeCampConfig.client?.landing?.[locale]?.title;
  return (
    <>
      {title && <h1>{title}</h1>}
      <p className="description">
        {
          // @ts-expect-error TODO
          freeCodeCampConfig.client?.landing?.[locale]?.description
        }
      </p>
      <a
        className="faq"
        // @ts-expect-error TODO
        href={freeCodeCampConfig.client?.landing?.[locale]?.["faq-link"]}
      >
        {
          // @ts-expect-error TODO
          freeCodeCampConfig.client?.landing?.[locale]?.["faq-text"]
        }
      </a>
      <Selection {...{ sock, projects }} />
    </>
  );
};
