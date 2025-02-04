import { useEffect, useState } from "react";

// Changes the Reset button background to a filling progress bar when the seed is running
function progressStyle(loader?: LoaderT) {
  if (!loader) {
    return {};
  }

  const {
    isLoading,
    progress: { total, count },
  } = loader;
  if (isLoading) {
    return {
      background: `linear-gradient(to right, #0065A9 ${
        (count / total) * 100
      }%, rgba(0,0,0,0) 0%)`,
    };
  }
}

export const Controls = () => {
  const [isTestsRunning, setIsTestsRunning] = useState(false);

  useEffect(() => {
    if (tests.some((t) => t.isLoading)) {
      setIsTestsRunning(true);
    } else {
      setIsTestsRunning(false);
    }
  }, [tests]);

  function handleTests() {
    if (isTestsRunning) {
      cancelTests();
    } else {
      runTests();
    }
  }

  const resetDisabled = !isResetEnabled || loader?.isLoading;

  return (
    <section className="project-controls">
      <button className="secondary-cta" onClick={handleTests}>
        {isTestsRunning ? "Cancel Tests" : "Run Tests"}
      </button>
      {resetProject && (
        <button
          disabled={resetDisabled}
          style={{
            ...progressStyle(loader),
            cursor: resetDisabled ? "not-allowed" : "pointer",
          }}
          onClick={() => resetProject()}
        >
          Reset Step
        </button>
      )}
    </section>
  );
};
