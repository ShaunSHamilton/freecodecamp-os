import { useState } from "react";
import { Header } from "./components/header";
import { Landing } from "./templates/landing";
import { Project } from "./types";

export function App() {
  const [project, setProject] = useState<Project | null>(null);

  return (
    <>
      <Header />
      <Landing />
    </>
  );
}
