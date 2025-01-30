import { useQuery } from "@tanstack/react-query";

import { FreeCodeCampConf, Lesson, Project } from "../types";
import { Description } from "../components/description";
import { Heading } from "../components/heading";
import "./project.css";

export const ProjectLesson = ({ project_id, lesson_id }) => {
  const { isPending, isError, data, error } = useQuery({
    queryKey: ["project", project_id, lesson_id],
    queryFn: async () => projectLoader({ project_id, lesson_id }),
  });

  if (isPending) {
    return <h1>Loading</h1>;
  }

  if (isError) {
    return <h1>{error.message}</h1>;
  }

  const { project, lesson, config } = data;
  return (
    <>
      <div className="container">
        <Heading
          // {...(project.isIntegrated
          title={project.title}
          // : {
          //     goToNextLesson,
          //     goToPreviousLesson,
          //     numberOfLessons: project.numberOfLessons,
          //     title: project.title,
          //     lessonNumber,
          //   })}
        />

        <Description description={lesson.description} />

        {/* <Controls
          {...(project.isIntegrated
            ? {
                cancelTests,
                runTests,
                tests,
              }
            : {
                cancelTests,
                runTests,
                resetProject,
                isResetEnabled: project.isResetEnabled,
                tests,
                loader,
              })}
        /> */}

        {/* <Output {...{ hints, tests, cons }} /> */}
      </div>
    </>
  );
};

async function projectLoader({ project_id, lesson_id }) {
  const project: Project = await (await fetch(`/project/${project_id}`)).json();
  const config: FreeCodeCampConf = await (await fetch("/config")).json();
  const lesson: Lesson = await (
    await fetch(`/project/${project_id}/${lesson_id}`)
  ).json();

  return {
    project,
    config,
    lesson,
  };
}
