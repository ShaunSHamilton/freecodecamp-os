export const Hints = ({ hints }: { hints: string[] }) => {
  return (
    <ul style={{ listStyle: 'none' }}>
      {hints.map((hint, i) => (
        <HintElement key={i} {...{ hint, i }} />
      ))}
    </ul>
  );
};

const HintElement = ({ hint, i }: { hint: string; i: number }) => {
  const details = `<summary>Hint ${i + 1}</summary>
  
  ${hint}`;
  return (
    <div>
      <details
        dangerouslySetInnerHTML={{
          __html: details
        }}
      ></details>
    </div>
  );
};
