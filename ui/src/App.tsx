import { evalProgram } from "monkey_interpreter";
import { useEffect, useState } from "react";

function App() {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");
  const [time, setTime] = useState<number | undefined>(undefined);

  useEffect(() => {
    const start = performance.now();
    const result = evalProgram(input);
    const end = performance.now();

    setTime(end - start);

    setOutput(result);
  }, [input]);

  return (
    <div className="App">
      <h1>Monkey Interpreter</h1>
      <p>Took {time} ms to evaluate</p>
      <textarea value={input} onChange={(e) => setInput(e.target.value)} />
      <pre>{output}</pre>
    </div>
  );
}

export default App;
