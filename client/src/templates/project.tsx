import { useQuery } from "@tanstack/react-query";

import { Description } from "../components/description";
import { Heading } from "../components/heading";
import { getLesson, getProject } from "../utils/fetch";
import "./project.css";
import { Controls } from "../components/controls";

interface ProjectLessonProps {
  project_id: number;
  lesson_id: number;
}

export const ProjectLesson = ({
  project_id,
  lesson_id,
}: ProjectLessonProps) => {
  const { isPending, isError, data, error } = useQuery({
    queryKey: ["project", project_id, lesson_id],
    queryFn: async () => {
      const lesson = await getLesson({ project_id, lesson_id });
      const project = await getProject({ project_id });
      return { lesson, project };
    },
  });

  if (isPending) {
    return <h1>Loading</h1>;
  }

  if (isError) {
    return <h1>{error.message}</h1>;
  }

  return (
    <>
      <div className="container">
        <Heading
          // {...(project.isIntegrated
          title={data.project.title}
          // : {
          //     goToNextLesson,
          //     goToPreviousLesson,
          //     numberOfLessons: project.numberOfLessons,
          //     title: project.title,
          //     lessonNumber,
          //   })}
        />

        <Description description={data.lesson.description} />

        <Controls
        // {...(project.isIntegrated
        //   ? {
        //       cancelTests,
        //       runTests,
        //       tests,
        //     }
        //   : {
        //       cancelTests,
        //       runTests,
        //       resetProject,
        //       isResetEnabled: project.isResetEnabled,
        //       tests,
        //       loader,
        //     })}
        />

        {/* <Output {...{ hints, tests, cons }} /> */}
      </div>
    </>
  );
};
