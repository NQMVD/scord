import { useQuery } from "convex/react";
import { api } from "../convex/_generated/api";
import { Toaster } from "sonner";
import { SpreadsheetView } from "./SpreadsheetView";

export default function App() {
  return (
    <div className="min-h-screen flex flex-col bg-charcoal-950 text-charcoal-100">
      <header className="sticky top-0 z-10 charcoal-header h-16 flex justify-between items-center px-4">
        <h1 className="text-3xl font-semibold charcoal-glow-text">
          {" "}
          {">"} scord
        </h1>
        <div className="flex gap-4 items-center">
          <a
            href="https://home.stardive.space"
            target="_blank"
            rel="noopener noreferrer"
            className="text-charcoal-300 hover:text-charcoal-100 transition-colors interactive-element px-3 py-1 rounded"
          >
            Home
          </a>
          <a
            href="https://github.com/nqmvd/scord"
            target="_blank"
            rel="noopener noreferrer"
            className="text-charcoal-300 hover:text-charcoal-100 transition-colors interactive-element px-3 py-1 rounded"
          >
            GitHub
          </a>
        </div>
      </header>
      <main className="flex-1 p-4">
        <SpreadsheetView />
      </main>
      <Toaster theme="dark" />
    </div>
  );
}
