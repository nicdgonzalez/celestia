"use client";

import { Pause, Play, RotateCcw } from "lucide-react";
import type React from "react";
import type { PropsWithChildren } from "react";

import { useWebSocket } from "@/components/WebSocketProvider";
import { cn } from "@/lib/utils";

export function Controls() {
  const { isOnline } = useWebSocket();

  return (
    <section
      id="controls"
      className="flex flex-row gap-x-8 items-center"
    >
      <ControlButton
        disabled={isOnline}
        SvgIcon={Play}
      >
        Start
      </ControlButton>
      <ControlButton
        className="hover:cursor-pointer disabled:hover:cursor-not-allowed"
        disabled={!isOnline}
        SvgIcon={Pause}
      >
        Stop
      </ControlButton>
      <ControlButton
        className="hover:cursor-pointer disabled:hover:cursor-not-allowed"
        disabled={!isOnline}
        SvgIcon={RotateCcw}
      >
        Restart
      </ControlButton>
      <p
        className={`ml-auto flex flex-row items-center gap-x-2 px-4 py-1 before:rounded-full before:content-[''] before:h-2 before:w-2 before:block ${
          isOnline ? "before:bg-green-500" : "before:bg-red-500"
        }`}
      >
        {isOnline ? "Online" : "Offline"}
      </p>
    </section>
  );
}

interface ControlButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>, PropsWithChildren {
  SvgIcon: React.FC<React.SVGAttributes<SVGElement>>;
}

function ControlButton(
  { SvgIcon, title, children, className, ...props }: ControlButtonProps,
) {
  return (
    <button
      type="button"
      className={cn(
        "flex flex-row items-center gap-x-2 hover:cursor-pointer disabled:hover:cursor-not-allowed",
        className,
      )}
      {...props}
    >
      <SvgIcon className="w-4 h-4" />
      <span>{children}</span>
    </button>
  );
}
