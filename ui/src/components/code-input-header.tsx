import { Dispatch, SetStateAction } from "react";
import { useMonaco } from "@monaco-editor/react";
import { Link2, Pause, Play, Sparkles, X } from "lucide-react";
import { toast } from "sonner";

import { cn } from "@/lib/utils";
import { Badge } from "@/components/ui/badge";

const CodeInputHeader = ({
  hasInput,
  clearInput,
  autoRun,
  setAutoRun,
  executeCode,
}: {
  hasInput: boolean;
  clearInput: () => void;
  autoRun: boolean;
  setAutoRun: Dispatch<SetStateAction<boolean>>;
  executeCode: () => void;
}) => {
  const monaco = useMonaco();

  return (
    <div className="flex justify-between gap-2 border-b border-border bg-background p-2">
      <div className="flex flex-wrap items-center gap-2">
        <Badge
          className={cn(hasInput ? "cursor-pointer" : "opacity-20")}
          variant="destructive"
          onClick={() => {
            if (!hasInput) {
              return;
            }
            clearInput();
            monaco?.editor.getModels().forEach((model) => {
              model.setValue("");
            });
          }}
        >
          <X size={16} />
          Clear
        </Badge>
        <Badge
          variant="outline"
          className="cursor-pointer"
          onClick={() => {
            navigator.clipboard.writeText(window.location.href);
            toast.success("Copied link to clipboard", {
              id: "share",
              description: "Created a link with the current code",
              className:
                "border-border border bg-background/50 text-foreground backdrop-blur-lg",
            });
          }}
        >
          <Link2 size={16} />
          Share
        </Badge>
      </div>
      <div className="flex flex-wrap items-center justify-end gap-2">
        <Badge
          variant="outline"
          className={cn(autoRun ? "opacity-20" : "cursor-pointer")}
          onClick={() => {
            if (!autoRun) {
              executeCode();
            }
          }}
        >
          <Sparkles size={16} />
          Run (Ctrl+Enter)
        </Badge>
        <Badge
          variant="outline"
          className="cursor-pointer"
          onClick={() => {
            setAutoRun((p) => !p);
          }}
        >
          {autoRun ? <Pause size={16} /> : <Play size={16} />}
          {autoRun ? "Pause" : "Resume"}
          <span className="text-xs">Auto-Run</span>
        </Badge>
      </div>
    </div>
  );
};

export default CodeInputHeader;
