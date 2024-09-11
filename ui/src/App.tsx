import { useCallback, useEffect, useState } from "react";
import { EvaluationResult, Token, execute, lexer } from "monkey-interpreter";

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

    const start = performance.now();
    const result = execute(input) as EvaluationResult;
    const end = performance.now();

    setTime(end - start);

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

  return (
    <div className="m-4 flex flex-1 flex-col rounded-3xl border border-border bg-background/50 shadow-md">
      <Header />
      <ExamplesSubHeader setInput={setInput} />
      <ResizablePanelGroup direction="horizontal" className="p-2 pt-0">
        <ResizablePanel
          minSize={30}
          className="flex flex-col rounded-s-2xl border border-e-0 border-border bg-transparent shadow-inner"
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
          className="flex-1 overflow-hidden rounded-e-2xl border border-s-0 border-border bg-background/50 text-sm shadow-inner"
          minSize={30}
        >
          <ResultsHeader tab={tab} setTab={setTab} time={time} />
          <ResultsContent tab={tab} results={results} tokens={tokens} />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
