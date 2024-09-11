import { Dispatch, SetStateAction } from "react";
import { Editor } from "@monaco-editor/react";

const CodeInputContent = ({
  input,
  setInput,
  theme,
}: {
  input?: string;
  setInput: Dispatch<SetStateAction<string | undefined>>;
  theme: "light" | "dark";
}) => (
  <Editor
    defaultLanguage="monkey"
    theme={theme === "dark" ? "vs-dark" : undefined}
    options={{
      minimap: {
        enabled: false,
      },
    }}
    value={input}
    onChange={(value) => setInput(value)}
  />
);

export default CodeInputContent;
