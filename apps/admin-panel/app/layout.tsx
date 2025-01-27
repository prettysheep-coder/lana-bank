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
import "./globals.css"
import { env } from "@/env"

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
        <BreadcrumbProvider>
          <ApolloProvider client={client}>
            <Authenticated appChildren={children}>
              <Toast />
              <SidebarProvider>
                <AppSidebar appVersion={appVersion} />
                <SidebarInset className="min-h-screen md:peer-data-[variant=inset]:shadow-none border">
                  <AppLayout>
                    <CommandMenu />
                    {children}
                  </AppLayout>
                </SidebarInset>
              </SidebarProvider>
            </Authenticated>
          </ApolloProvider>
        </BreadcrumbProvider>
      </body>
    </html>
  )
}

export default RootLayout
