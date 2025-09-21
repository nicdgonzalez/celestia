import { Controls } from "./_sections/Controls";

export default function Home() {
  return (
    <main className="flex flex-col min-h-screen w-full gap-y-8 px-8 py-4">
      <header className="flex flex-col">
        <h1 className="text-4xl font-bold">Overview</h1>
        <h2 className="text-xl text-slate-700 dark:text-slate-300">
          Monitor your server.
        </h2>
      </header>
      <Controls />
    </main>
  );
}
