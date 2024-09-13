import { EvaluationResult, Token } from "@benjeau/monkey-interpreter";

const ResultsContent = ({
  tab,
  results,
  tokens,
  interpreterCrashed,
}: {
  tab: number;
  results?: EvaluationResult;
  tokens: Token[];
  interpreterCrashed: boolean;
}) => (
  <pre className="overflow-auto whitespace-break-spaces p-2 shadow-inner">
    {tab === 0 && (
      <div>
        {(results?.errors.length ?? 0) > 0 && (
          <>
            <h3 className="font-bold text-red-500">
              {interpreterCrashed ? "Interpreter Crashed!" : "Errors"}{" "}
            </h3>
            <ul className="list-inside list-disc">
              {results?.errors.map((error, index) => (
                <li key={index}>{error}</li>
              ))}
            </ul>
            <hr className="m-4 border-border" />
          </>
        )}
        {results?.output ? (
          results.output
        ) : (
          <span className="italic opacity-50">No output</span>
        )}
      </div>
    )}
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
      (results?.program ?? (
        <span className="italic opacity-50">No parsed program</span>
      ))}
  </pre>
);

export default ResultsContent;
