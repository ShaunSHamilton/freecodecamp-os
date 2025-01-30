import { Description } from "../components/description";
import { Heading } from "../components/heading";
import { FreeCodeCampConf, Lesson, Project } from "../types";
import "./project.css";
import { getRouteApi, useParams } from "@tanstack/react-router";

export const ProjectLesson = () => {
  const params = useParams({ strict: false });

  const routeApi = getRouteApi(
    `/project/${params.project_id}/${params.lesson_id}`
  );
  const { project, lesson, config } = routeApi.useLoaderData();
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

export const projectLoader = async ({ params: { project_id, lesson_id } }) => {
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
};
