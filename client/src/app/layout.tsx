import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";

import { WebSocketProvider } from "@/components/WebSocketProvider";

import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Minecraft Dashboard",
  description: "",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const websocketUrl = await fetch("http://127.0.0.1:1140/api/gateway")
    .then(async (response) => await response.json())
    .then((data) => data.url);

  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-zinc-100 dark:bg-zinc-900 text-black dark:text-white`}
      >
        <WebSocketProvider url={websocketUrl}>
          {children}
        </WebSocketProvider>
      </body>
    </html>
  );
}
