import { F, parseMarkdown } from "../utils";
// import { Checkmark } from "./checkmark";
import { Project } from "../types";
// import { Tag } from "./tag";

type BlockProps = Project & { set_project_id: F<number, void> };

export const Block = ({
  id,
  is_public,
  description,
  title,
  set_project_id,
}: // isIntegrated,
// numberOfLessons,
// currentLesson,
// completedDate,
// tags,
BlockProps) => {
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
      <button
        className="block-btn"
        onClick={() => set_project_id(id)}
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
    </li>
  );
};
