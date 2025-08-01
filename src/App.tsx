import { useQuery } from "convex/react";
import { api } from "../convex/_generated/api";
import { Toaster } from "sonner";
import { SpreadsheetView } from "./SpreadsheetView";

export default function App() {
  return (
    <div className="min-h-screen flex flex-col bg-charcoal-950 text-charcoal-100">
      <header className="sticky top-0 z-10 charcoal-header h-16 flex justify-between items-center px-4">
        <h2 className="text-xl font-semibold charcoal-glow-text">Contestant Scoring</h2>
      </header>
      <main className="flex-1 p-4">
        <SpreadsheetView />
      </main>
      <Toaster theme="dark" />
    </div>
  );
}
