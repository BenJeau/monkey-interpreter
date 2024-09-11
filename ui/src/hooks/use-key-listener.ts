import { useEffect } from "react";

export const useKeyListener = (key: string, callback: () => void) => {
  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.ctrlKey || event.metaKey) {
        if (event.key === key) {
          event.preventDefault();
          callback();
        }
      }
    }

    document.body.addEventListener("keydown", handleKeyDown);

    return () => {
      document.body.removeEventListener("keydown", handleKeyDown);
    };
  }, [key, callback]);
};
