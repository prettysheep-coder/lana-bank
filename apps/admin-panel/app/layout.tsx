// eslint-disable-next-line import/no-unassigned-import
import "@/lib/ui/globals.css"
import type { Metadata } from "next"

import { HelveticaNeueFont, RobotoMono } from "@/lib/ui/fonts"
import { Toast } from "@/components/toast"

export const metadata: Metadata = {
  title: "Lana Bank | Admin Panel",
  icons: [
    {
      rel: "icon",
      url: "/favicon.ico",
      type: "image/x-icon",
    },
  ],
}

const RootLayout: React.FC<React.PropsWithChildren> = async ({ children }) => {
  return (
    <html lang="en">
      <body
        className={`${HelveticaNeueFont.variable} ${RobotoMono.variable} antialiased w-screen h-screen select-none`}
      >
        <Toast />
        {children}
      </body>
    </html>
  )
}

export default RootLayout
