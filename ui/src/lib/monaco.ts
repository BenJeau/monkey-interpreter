import { loader } from "@monaco-editor/react";
import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";

import { language, conf } from "@/lib/monaco-monkey-lang-definition.ts";

self.MonacoEnvironment = {
  getWorker() {
    return new editorWorker();
  },
};

monaco.languages.register({ id: "monkey" });
monaco.languages.setLanguageConfiguration("monkey", conf);
monaco.languages.setMonarchTokensProvider("monkey", language);

loader.config({ monaco });
