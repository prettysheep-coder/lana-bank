"use client"

import { useEffect, useState, useCallback } from "react"
import { usePathname, useRouter } from "next/navigation"

import { AppLayout } from "../app-layout"

import { getSession, logoutUser } from "./ory"

type Props = {
  children: React.ReactNode
}

export const Authenticated: React.FC<Props> = ({ children }) => {
  const router = useRouter()
  const pathName = usePathname()

  const isAuthSetInLocalStorage = localStorage.getItem("isAuthenticated")

  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null)
  useEffect(() => {
    ;(async () => {
      try {
        await getSession()
        setIsAuthenticated(true)
        localStorage.setItem("isAuthenticated", "true")
        if (pathName === "/") router.push("/dashboard")
      } catch (error) {
        setIsAuthenticated(false)
        if (!pathName.startsWith("/auth")) router.push("/auth/login")
      }
    })()
  }, [pathName, router])

  if (isAuthenticated && isAuthSetInLocalStorage) return <AppLayout>{children}</AppLayout>
  else if (!isAuthenticated && isAuthSetInLocalStorage)
    return <AppLayout>{children}</AppLayout>
  else return <main className="h-screen w-full flex flex-col">{children}</main>
}

export const useLogout = () => {
  const router = useRouter()

  const logout = useCallback(async () => {
    await logoutUser()
    localStorage.removeItem("isAuthenticated")
    router.push("/")
  }, [router])

  return { logout }
}
