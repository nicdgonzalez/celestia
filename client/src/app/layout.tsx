import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";

import { WebSocketProvider } from "@/components/WebSocketProvider";

import "./globals.css";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Navigation } from "@/components/Navigation";

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

  const isOnline = await fetch("http://127.0.0.1:1140/api/server/status")
    .then(async (response) => await response.json())
    .then((data) => data.is_online);

  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-slate-100 dark:bg-slate-900 text-black dark:text-white`}
      >
        <WebSocketProvider url={websocketUrl} isOnline={isOnline}>
          <Header />
          <div className="flex flex-row">
            <Navigation />
            {children}
          </div>
          <Footer />
        </WebSocketProvider>
      </body>
    </html>
  );
}
