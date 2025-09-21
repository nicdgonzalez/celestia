import { LayoutDashboard, SlidersHorizontal, Terminal } from "lucide-react";

export function Navigation() {
  const navigationEntries: NavigationEntryProps[] = [
    {
      SvgIcon: LayoutDashboard,
      href: "/",
      title: "Overview",
    },
    {
      SvgIcon: Terminal,
      href: "/console",
      title: "Console",
    },
    {
      SvgIcon: SlidersHorizontal,
      href: "/properties",
      title: "Server Properties",
    },
  ];

  return (
    <nav className="sticky left-0 flex flex-col h-screen min-w-64 px-4 py-4 border-r border-r-slate-500">
      <ul className="flex flex-col gap-y-2">
        {navigationEntries.map((entry) => (
          <NavigationEntry key={entry.title.toLowerCase()} {...entry} />
        ))}
      </ul>
    </nav>
  );
}

interface NavigationEntryProps {
  SvgIcon: React.FC<React.SVGAttributes<SVGElement>>;
  href: string;
  title: string;
}

function NavigationEntry({ SvgIcon, href, title }: NavigationEntryProps) {
  return (
    <li>
      <a
        href={href}
        className="items-center text-sm px-4 py-2 hover:bg-slate-200 dark:hover:bg-slate-800 data-[selected]:bg-slate-200 dark:data-[selected]:bg-slate-800 data-[selected]:font-semibold rounded flex flex-row gap-x-2"
      >
        <SvgIcon className="w-4 h-4" />
        <span>{title}</span>
      </a>
    </li>
  );
}
