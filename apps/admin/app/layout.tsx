// eslint-disable-next-line import/no-unassigned-import
import "./globals.css"

import type { Metadata } from "next"

export const metadata: Metadata = {
  title: "Lava Bank | Admin",
  description:
    "Comprehensive banking management system: oversee financials, manage customers, approve loans, and ensure government compliance with seamless reporting",
  icons: {
    icon: "/favicon.ico",
  },
}

import { getServerSession } from "next-auth"
import { redirect } from "next/navigation"

import { Inter, Helvetica } from "./fonts"
import { AuthSessionProvider } from "./session-provider"

import { authOptions } from "./api/auth/[...nextauth]/options"

const RootLayout: React.FC<React.PropsWithChildren> = async ({ children }) => {
  const session = await getServerSession(authOptions)
  if (!session) {
    redirect("/api/auth/signin")
  }

  return (
    <html lang="en">
      <body className={`${Inter.variable} ${Helvetica.variable}`}>
        <AuthSessionProvider session={session}>{children}</AuthSessionProvider>
      </body>
    </html>
  )
}

export default RootLayout
