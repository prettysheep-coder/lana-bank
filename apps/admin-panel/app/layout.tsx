"use client"

import { Inter_Tight } from "next/font/google"

import { ApolloProvider } from "@apollo/client"

import { AppLayout } from "./app-layout"

import { CommandMenu } from "./command-menu"
import { Authenticated } from "./auth/session"
import { BreadcrumbProvider } from "./breadcrumb-provider"

import { makeClient } from "@/lib/apollo-client/client"
import { Toast } from "@/components/toast"
import { SidebarProvider, SidebarInset } from "@/ui/sidebar"
import { AppSidebar } from "@/components/app-sidebar"

// eslint-disable-next-line import/no-unassigned-import
import { env } from "@/env"
import "./globals.css"

const inter = Inter_Tight({
  subsets: ["latin"],
  variable: "--font-inter",
})

const RootLayout: React.FC<React.PropsWithChildren> = ({ children }) => {
  const appVersion = env.NEXT_PUBLIC_APP_VERSION
  const client = makeClient({
    coreAdminGqlUrl: appVersion.endsWith("dev") ? "/admin/graphql" : "/graphql",
  })

  return (
    <html lang="en">
      <body className={`${inter.className} antialiased bg-background`}>
        <ApolloProvider client={client}>
          <Toast />
          <Authenticated>{children}</Authenticated>
        </ApolloProvider>
      </body>
    </html>
  )
}

export default RootLayout
