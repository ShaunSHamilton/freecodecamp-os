import { Tag } from "./tag";
import { Checkmark } from "./checkmark";
import { Project } from "../types";
import { parseMarkdown } from "../utils";
import { Link } from "@tanstack/react-router";

type BlockProps = Project;

export const Block = ({
  meta: { id, is_public },
  description,
  title,
  // isIntegrated,
  // numberOfLessons,
  // currentLesson,
  // completedDate,
  // tags,
}: BlockProps) => {
  // let lessonsCompleted = 0;
  // if (completedDate) {
  //   lessonsCompleted = numberOfLessons;
  // } else {
  //   lessonsCompleted =
  //     !isIntegrated && currentLesson === numberOfLessons - 1
  //       ? currentLesson + 1
  //       : currentLesson;
  // }
  return (
    <li className="block">
      <Link
        to={`/project/$project_id/$lesson_id`}
        params={{ project_id: id, lesson_id: 0 }}
      >
        <button
          className="block-btn"
          disabled={!is_public}
          style={
            !is_public
              ? {
                  cursor: "not-allowed",
                }
              : {}
          }
        >
          <div className={"tags-row"}>
            {/* {tags.map((text) => {
            return <Tag text={text} />;
          })} */}
          </div>

          <h2>
            {title}
            {/* {completedDate ? (
            <span className="block-checkmark">
              <Checkmark />
            </span>
          ) : null} */}
          </h2>
          <div className="block-info">
            <p
              dangerouslySetInnerHTML={{
                __html: parseMarkdown(description),
              }}
            ></p>
            <span aria-hidden="true">
              {/* {lessonsCompleted}/{numberOfLessons} */}
            </span>
            <span className="sr-only">
              {/* {lessonsCompleted} of {numberOfLessons} lessons completed */}
            </span>
          </div>
        </button>
      </Link>
    </li>
  );
};
