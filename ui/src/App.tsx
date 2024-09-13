import { useCallback, useEffect, useState } from "react";
import {
  EvaluationResult,
  Token,
  execute,
  lexer,
} from "@benjeau/monkey-interpreter";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import Header from "@/components/header";
import ExamplesSubHeader from "@/components/examples-sub-header";
import CodeInputHeader from "@/components/code-input-header";
import CodeInputContent from "@/components/code-input-content";
import ResultsHeader from "@/components/results-header";
import ResultsContent from "@/components/results-content";
import { useKeyListener } from "@/hooks/use-key-listener";
import { useWindowWidth } from "@/hooks/use-window-width";
import { useTheme } from "@/lib/utils";

function App() {
  const theme = useTheme();

  const [time, setTime] = useState<number>(0);
  const [autoRun, setAutoRun] = useState(true);
  const [tab, setTab] = useState<0 | 1 | 2 | 3>(0);
  const [input, setInput] = useState<undefined | string>(() => {
    const hash = window.location.hash;
    if (hash.length === 0) {
      return undefined;
    }
    return atob(hash.slice(1));
  });

  const [interpreterCrashed, setInterpreterCrashed] = useState(false);
  const [tokens, setTokens] = useState<Token[]>([]);
  const [results, setResults] = useState<EvaluationResult | undefined>(
    undefined,
  );

  const executeCode = useCallback(() => {
    if (!input) {
      setTokens([]);
      setResults(undefined);
      return;
    }

    setTokens(lexer(input));

    let result: EvaluationResult | undefined = undefined;
    let crashed = false;

    const start = performance.now();
    try {
      result = execute(input) as EvaluationResult;
    } catch (error) {
      const errors = [];

      if (error instanceof Error) {
        errors.push(error.name + ": " + error.message);
      } else {
        errors.push("unknown error");
      }

      result = {
        statements: [],
        program: input,
        errors: errors,
        environment: undefined,
        output: undefined,
      };
      crashed = true;
    }
    const end = performance.now();

    setTime(end - start);

    setInterpreterCrashed(crashed);
    setResults(result);
  }, [input]);

  useEffect(() => {
    if (!input) {
      window.location.hash = "";
      setTokens([]);
      setResults(undefined);
      return;
    }

    window.location.hash = btoa(input);

    if (autoRun) {
      executeCode();
    }
  }, [input, executeCode, autoRun]);

  useKeyListener("Enter", executeCode);
  const windowWidth = useWindowWidth();

  return (
    <div className="m-2 flex flex-1 flex-col rounded-3xl border border-border bg-background/50 shadow-md md:m-4">
      <Header />
      <ExamplesSubHeader setInput={setInput} />
      <ResizablePanelGroup
        direction={windowWidth > 768 ? "horizontal" : "vertical"}
        className="p-2 pt-0"
      >
        <ResizablePanel
          minSize={30}
          className="flex flex-col rounded-t-2xl border border-b-0 border-border bg-transparent shadow-inner md:rounded-none md:rounded-s-2xl md:border-b md:border-e-0"
        >
          <CodeInputHeader
            hasInput={input !== undefined}
            clearInput={() => {
              setInput(undefined);
            }}
            autoRun={autoRun}
            setAutoRun={setAutoRun}
            executeCode={executeCode}
          />
          <CodeInputContent input={input} setInput={setInput} theme={theme} />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel
          className="flex-1 overflow-hidden rounded-b-2xl border border-t-0 border-border bg-background/50 text-sm shadow-inner md:rounded-none md:rounded-e-2xl md:border-s-0 md:border-t"
          minSize={30}
        >
          <ResultsHeader
            tab={tab}
            setTab={setTab}
            time={time}
            numberOfErrors={results?.errors.length ?? 0}
          />
          <ResultsContent
            tab={tab}
            results={results}
            tokens={tokens}
            interpreterCrashed={interpreterCrashed}
          />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
