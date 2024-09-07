import { parseProgram } from "monkey_interpreter";
import { useEffect, useState } from "react";

function App() {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  useEffect(() => {
    setOutput(parseProgram(input));
  }, [input]);

  return (
    <div className="App">
      <h1>Monkey Interpreter</h1>
      <textarea value={input} onChange={(e) => setInput(e.target.value)} />
      <pre>{output}</pre>
    </div>
  );
}

export default App;
