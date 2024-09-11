import { ChartNoAxesGantt, Code, ListTree, Terminal } from "lucide-react";

import { Badge } from "@/components/ui/badge";

const ResultsHeader = ({
  tab,
  setTab,
  time,
}: {
  tab: number;
  setTab: (tab: 0 | 1 | 2 | 3) => void;
  time: number;
}) => (
  <div className="flex justify-between gap-2 border-b border-border bg-background p-2">
    <div className="flex flex-wrap gap-2">
      <Badge
        variant={tab === 0 ? "default" : "outline"}
        className="cursor-pointer"
        onClick={() => setTab(0)}
      >
        <Terminal size={16} />
        Output
      </Badge>
      <Badge
        variant={tab === 1 ? "default" : "outline"}
        className="cursor-pointer"
        onClick={() => setTab(1)}
      >
        <ChartNoAxesGantt size={16} />
        Lexer
      </Badge>
      <Badge
        variant={tab === 2 ? "default" : "outline"}
        className="cursor-pointer"
        onClick={() => setTab(2)}
      >
        <ListTree size={16} />
        AST
      </Badge>
      <Badge
        variant={tab === 3 ? "default" : "outline"}
        className="cursor-pointer"
        onClick={() => setTab(3)}
      >
        <Code size={16} />
        Parser
      </Badge>
    </div>
    <div className="flex items-center gap-2 text-xs">
      <p className="whitespace-nowrap">No errors</p>
      <Badge variant="secondary">{time} ms</Badge>
    </div>
  </div>
);

export default ResultsHeader;
