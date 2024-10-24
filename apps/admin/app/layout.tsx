// eslint-disable-next-line import/no-unassigned-import
import "./globals.css"

import type { Metadata } from "next"
import { getServerSession } from "next-auth"
import { redirect } from "next/navigation"
import { headers } from "next/headers"

import { Inter, Helvetica } from "./fonts"
import { AuthSessionProvider } from "./session-provider"
import { authOptions } from "./api/auth/[...nextauth]/options"

const PUBLIC_PAGES = ["/auth/signin", "/auth/error", "/auth/verify"]

const RootLayout: React.FC<React.PropsWithChildren> = async ({ children }) => {
  const headerList = headers()
  const currentPath = headerList.get("x-current-path") || "/"

  const session = await getServerSession(authOptions)
  if (!session && !PUBLIC_PAGES.includes(currentPath)) redirect("/auth/signin")
  if (session && PUBLIC_PAGES.includes(currentPath)) redirect("/")

  return (
    <html lang="en">
      <body className={`${Inter.variable} ${Helvetica.variable}`}>
        <AuthSessionProvider session={session}>{children}</AuthSessionProvider>
      </body>
    </html>
  )
}

export default RootLayout

export const metadata: Metadata = {
  title: "Lava Bank | Admin",
  description:
    "Comprehensive banking management system: oversee financials, manage customers, approve loans, and ensure government compliance with seamless reporting",
  icons: {
    icon: "/favicon.ico",
  },
}
