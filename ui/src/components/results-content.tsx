import { EvaluationResult, Token } from "@benjeau/monkey-interpreter";

const ResultsContent = ({
  tab,
  results,
  tokens,
}: {
  tab: number;
  results?: EvaluationResult;
  tokens: Token[];
}) => (
  <pre className="overflow-auto p-2 shadow-inner">
    {tab === 0 &&
      (results?.output ? (
        results.output
      ) : (
        <span className="italic opacity-50">No output</span>
      ))}
    {tab === 1 && (
      <>
        {tokens.map((token, index) => (
          <p key={index}>
            {token.kind}{" "}
            {"value" in token && (
              <span className="opacity-50">{token.value}</span>
            )}
          </p>
        ))}
        {tokens.length === 0 && (
          <span className="italic opacity-50">No tokens</span>
        )}
      </>
    )}
    {tab === 2 && (
      <>
        {results?.statements.map((statement, index) => (
          <p key={index}>
            <span className="opacity-50">{statement.kind}</span>{" "}
            {"name" in statement.value && statement.value.name + " "}
            {JSON.stringify(statement.value.value, null, 2)}
          </p>
        ))}
        {!results?.statements && (
          <span className="italic opacity-50">No statements</span>
        )}
      </>
    )}
    {tab === 3 &&
      (results?.program ? (
        results.program.split(";").join(";\n")
      ) : (
        <span className="italic opacity-50">No parsed program</span>
      ))}
  </pre>
);

export default ResultsContent;
