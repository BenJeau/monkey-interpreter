/**
 * Parses and executes the provided Monkey code.
 * @param {string} input
 * @returns {EvaluationResult[]}
 */
export function execute(input: string): EvaluationResult;

/**
 * Tokenizes the provided Monkey code.
 * @param {string} input
 * @returns {Token[]}
 */
export function lexer(input: string): Token[];

interface EvaluationResult {
  statements: Statement[];
  program: string;
  errors: string[];
  environment?: Environment;
  output?: string;
}

type Expression =
  | {
      kind: "integer";
      value: number;
    }
  | {
      kind: "identifier";
      value: string;
    }
  | {
      kind: "boolean";
      value: boolean;
    }
  | {
      kind: "string";
      value: string;
    }
  | {
      kind: "prefix_operator";
      value: {
        operator: string;
        expression: Expression;
      };
    }
  | {
      kind: "infix_operator";
      value: {
        operator: string;
        lh_expression: Expression;
        rh_expression: Expression;
      };
    }
  | {
      kind: "function_call";
      value: {
        name: Expression;
        arguments: Expression[];
      };
    }
  | {
      kind: "if";
      value: {
        condition: Expression;
        consequence: Statement[];
        alternative?: Statement[];
      };
    }
  | {
      kind: "function";
      value: {
        arguments: string[];
        body: Statement[];
      };
    }
  | {
      kind: "array";
      value: Expression[];
    }
  | {
      kind: "index";
      value: {
        left: Expression;
        index: Expression;
      };
    }
  | {
      kind: "hash_literal";
      value: { key: Expression; value: Expression }[];
    };

type Statement =
  | {
      kind: "let";
      value: {
        name: string;
        value: Expression;
      };
    }
  | {
      kind: "return";
      value: Expression;
    }
  | {
      kind: "expression";
      value: Expression;
    };

interface Environment {
  store: Record<string, Object>;
  parent?: Environment;
}

type Object =
  | {
      kind: "interger";
      value: number;
    }
  | {
      kind: "boolean";
      value: boolean;
    }
  | {
      kind: "string";
      value: string;
    }
  | {
      kind: "return";
      value: Object;
    }
  | {
      kind: "error";
      value: string;
    }
  | {
      kind: "function";
      value: {
        parameters: string[];
        environment: Environment;
        body: Statement[];
      };
    }
  | {
      kind: "array";
      value: Object[];
    }
  | {
      kind: "hash";
      value: Record<Object, Object>;
    }
  | {
      kind: "null";
    };

type Token =
  | {
      kind: "integer";
      value: number;
    }
  | {
      kind: "identifier";
      value: string;
    }
  | {
      kind: "string";
      value: string;
    }
  | {
      kind: "illegal";
      value: string;
    }
  | {
      kind:
        | "equal_sign"
        | "plus_sign"
        | "minus_sign"
        | "exclamation_mark"
        | "asterisk"
        | "slash"
        | "less_than"
        | "greater_than"
        | "equal"
        | "not_equal"
        | "comma"
        | "semicolon"
        | "left_paren"
        | "right_paren"
        | "left_brace"
        | "right_brace"
        | "left_bracket"
        | "right_bracket"
        | "colon"
        | "function"
        | "true"
        | "false"
        | "if"
        | "else"
        | "let"
        | "return"
        | "eof";
    };
