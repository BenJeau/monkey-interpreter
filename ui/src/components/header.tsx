import { BookOpenText, Origami } from "lucide-react";

import { Npm, Github } from "@/components/icons";

const Header = () => (
  <header className="flex flex-row items-center justify-between p-3">
    <div className="flex flex-row items-center gap-4">
      <div className="group">
        <div className="rounded-xl bg-teal-200 p-4 transition-all ease-out group-hover:-scale-x-100 group-hover:bg-teal-600 group-hover:text-teal-50 group-hover:shadow-inner dark:bg-teal-950 dark:text-teal-300">
          <Origami />
        </div>
      </div>
      <div>
        <h1 className="text-2xl font-medium">
          Monkey Interpreter
          <span className="ms-2 text-xs font-normal">
            Made with &lt;3 by{" "}
            <a
              href="https://jeaurond.dev"
              target="_blank"
              rel="noreferrer noopener"
              className="underline hover:decoration-dotted"
            >
              @BenJeau
            </a>
          </span>
        </h1>
        <p className="text-sm opacity-50">
          An interpreter built in Rust and compiled to WebAssembly
        </p>
      </div>
    </div>
    <div className="flex flex-col items-end gap-1">
      <a
        href="https://github.com/BenJeau/monkey-interpreter"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer items-center gap-2 text-xs underline hover:decoration-dotted"
      >
        Source code
        <Github />
      </a>
      <a
        href="https://www.npmjs.com/package/@benjeau/monkey-interpreter"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer items-center gap-2 text-xs underline hover:decoration-dotted"
      >
        @benjeau/monkey-interpreter
        <Npm />
      </a>
      <a
        href="https://interpreterbook.com/"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer items-center gap-2 text-xs underline hover:decoration-dotted"
      >
        Writing An Interpreter In Go
        <BookOpenText size={15} />
      </a>
    </div>
  </header>
);

export default Header;
