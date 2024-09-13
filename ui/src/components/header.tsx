import { BookOpenText, Origami } from "lucide-react";

import { Npm, Github } from "@/components/icons";

const Header = () => (
  <header className="flex flex-col justify-between gap-4 p-3 md:flex-row md:items-center">
    <div className="flex flex-row items-center gap-4">
      <div className="group">
        <div className="rounded-xl bg-teal-200 p-4 transition-all ease-out group-hover:-scale-x-100 group-hover:bg-teal-600 group-hover:text-teal-50 group-hover:shadow-inner dark:bg-teal-950 dark:text-teal-300">
          <Origami />
        </div>
      </div>
      <div>
        <div className="flex flex-col items-baseline md:flex-row md:gap-2">
          <h1 className="text-2xl font-medium">Monkey Interpreter</h1>
          <span className="text-xs">
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
        </div>
        <p className="text-sm opacity-50">
          An interpreter built in Rust and compiled to WebAssembly
        </p>
      </div>
    </div>
    <div className="flex flex-wrap items-end gap-1 gap-x-3 sm:w-auto md:flex-col md:gap-x-1">
      <a
        href="https://github.com/BenJeau/monkey-interpreter"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer flex-row-reverse items-center gap-2 text-xs underline hover:decoration-dotted md:flex-row"
      >
        Source code
        <Github />
      </a>
      <a
        href="https://www.npmjs.com/package/@benjeau/monkey-interpreter"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer flex-row-reverse items-center gap-2 text-xs underline hover:decoration-dotted md:flex-row"
      >
        @benjeau/monkey-interpreter
        <Npm />
      </a>
      <a
        href="https://interpreterbook.com/"
        target="_blank"
        rel="noreferrer noopener"
        className="flex cursor-pointer flex-row-reverse items-center gap-2 text-xs underline hover:decoration-dotted md:flex-row"
      >
        Writing An Interpreter In Go
        <BookOpenText size={15} />
      </a>
    </div>
  </header>
);

export default Header;
