import FreeCodeCampLogo from "../assets/fcc_primary_large";
import { F } from "../utils";

interface HeaderProps {
  // updateProject: (project: ProjectI | null) => void;
  // freeCodeCampConfig: FreeCodeCampConfigI;
  // sock: (type: Events, data: unknown) => void;
  set_project_id: F<number | null, void>;
}
export const Header = ({
  // sock,
  // updateProject,
  // freeCodeCampConfig,
  set_project_id,
}: HeaderProps) => {
  // function returnToLanding() {
  //   updateProject(null);
  // }

  // const locales = freeCodeCampConfig?.curriculum?.locales
  //   ? // @ts-expect-error TODO
  //     Object.keys(freeCodeCampConfig?.curriculum?.locales)
  //   : [];
  return (
    <header>
      <button
        className="header-btn"
        onClick={() => {
          set_project_id(null);
        }}
      >
        <FreeCodeCampLogo />
      </button>
      {/* {locales.length > 1 ? <LanguageList {...{ sock, locales }} /> : null} */}
    </header>
  );
};
