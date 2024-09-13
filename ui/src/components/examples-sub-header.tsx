import { codeExamples } from "@/examples";

const ExamplesSubHeader = ({
  setInput,
}: {
  setInput: (code: string) => void;
}) => (
  <div className="m-2 mt-0 flex flex-wrap gap-x-2 gap-y-1 rounded-2xl border border-border bg-background/50 px-3 py-2 text-xs shadow-inner">
    Load code examples:
    {Object.entries(codeExamples).map(([name, code]) => (
      <a
        key={name}
        href={`#${btoa(code.trim())}`}
        onClick={(e) => {
          e.preventDefault();
          setInput(code.trim());
        }}
        className="underline hover:decoration-dotted"
      >
        {name}
      </a>
    ))}
  </div>
);

export default ExamplesSubHeader;
