import FreeCodeCampLogo from "../assets/fcc_primary_large";

interface HeaderProps {
  // updateProject: (project: ProjectI | null) => void;
  // freeCodeCampConfig: FreeCodeCampConfigI;
  // sock: (type: Events, data: unknown) => void;
}
export const Header = (
  {
    // sock,
    // updateProject,
    // freeCodeCampConfig,
  }: HeaderProps
) => {
  // function returnToLanding() {
  //   updateProject(null);
  // }

  // const locales = freeCodeCampConfig?.curriculum?.locales
  //   ? // @ts-expect-error TODO
  //     Object.keys(freeCodeCampConfig?.curriculum?.locales)
  //   : [];
  return (
    <header>
      <button className="header-btn" onClick={() => {}}>
        <FreeCodeCampLogo />
      </button>
      {/* {locales.length > 1 ? <LanguageList {...{ sock, locales }} /> : null} */}
    </header>
  );
};
