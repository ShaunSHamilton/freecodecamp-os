import { useState } from "react";
import { Header } from "./components/header";
import { Landing } from "./templates/landing";
import { useQuery } from "@tanstack/react-query";
import { getState } from "./utils/fetch";
import { ProjectLesson } from "./templates/project";

export function App() {
  const [project_id, set_project_id] = useState<number | null>(null);

  // If `project_id`, get current `lesson_id`
  const stateQuery = useQuery({
    queryKey: ["state", project_id],
    queryFn: getState,
  });

  if (project_id === null) {
    return (
      <>
        <Header {...{ set_project_id }} />
        <Landing {...{ set_project_id }} />
      </>
    );
  }

  // If `project_id`, check for completed lessons, and grab the highest lesson_id
  // If the `lesson_id` does not exist, set to 0
  if (stateQuery.data) {
    const lesson_id = stateQuery.data.completed_lessons
      .filter((cl) => cl.project_id === project_id)
      .reduce((acc, cl) => Math.max(acc, cl.lesson_id), 0);
    return (
      <>
        <Header {...{ set_project_id }} />
        <ProjectLesson {...{ project_id, lesson_id }} />
      </>
    );
  }
}
