import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "The Symbiotic Truth Engine | Vector Lens",
  description: "Spatial computing engine translating 1D math and physics into 3D topological geometries.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-void text-slate-100 antialiased h-screen w-screen overflow-hidden">
        {children}
      </body>
    </html>
  );
}
