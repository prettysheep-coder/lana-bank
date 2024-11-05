// eslint-disable-next-line import/no-unassigned-import
import "@/lib/ui/globals.css"
import type { Metadata } from "next"

export const metadata: Metadata = {
  title: "Lana Bank | Admin Panel",
  icons: [
    {
      rel: "icon",
      url: "/favicon.ico",
    },
  ],
}

const RootLayout = async ({
  children,
}: Readonly<{
  children: React.ReactNode
}>) => {
  return (
    <html lang="en">
      <body className="antialiased w-screen h-screen select-none">{children}</body>
    </html>
  )
}

export default RootLayout
