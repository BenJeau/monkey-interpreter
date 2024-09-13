import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Toaster } from "sonner";

import App from "@/App";
import "@/lib/monaco";
import "@/index.css";

const root = document.getElementById("root");

if (!root) {
  throw new Error("Root element not found");
}

const loadingContainer = document.getElementById("loading-container");
const loadingContent = document.getElementById("loading-content");

if (loadingContent) {
  loadingContent.classList.add("opacity-100");
}

if (loadingContainer) {
  setTimeout(() => {
    loadingContainer.style.background = "#00000000";
    setTimeout(() => {
      loadingContainer.style.opacity = "0";
      setTimeout(() => {
        loadingContainer.remove();
      }, 100);
    }, 200);
  }, 100);
}

createRoot(root).render(
  <StrictMode>
    <App />
    <Toaster />
  </StrictMode>,
);
