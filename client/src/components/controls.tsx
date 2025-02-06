import { Lesson, Project } from "../types";
import { runTests } from "../utils/fetch";

interface ControlsProps {
  project: Project;
  lesson: Lesson;
}

export function Controls({ project, lesson }: ControlsProps) {
  function handleTests() {
    runTests({ project_id: project.id, lesson_id: lesson.id });
  }

  return (
    <section className="project-controls">
      <button className="secondary-cta" onClick={handleTests}>
        Run Tests
      </button>
    </section>
  );
}
