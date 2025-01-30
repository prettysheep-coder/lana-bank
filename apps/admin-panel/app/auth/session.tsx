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

  const [isAuthSetInLocalStorage, setIsAuthSetInLocalStorage] = useState(false)
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null)

  useEffect(() => {
    if (typeof window !== "undefined") {
      const authFromLocalStorage = localStorage.getItem("isAuthenticated")
      setIsAuthSetInLocalStorage(!!authFromLocalStorage)
    }
  }, [])

  useEffect(() => {
    ;(async () => {
      try {
        await getSession()
        setIsAuthenticated(true)

        if (typeof window !== "undefined") {
          localStorage.setItem("isAuthenticated", "true")
        }
        if (pathName === "/") router.push("/dashboard")
      } catch (error) {
        setIsAuthenticated(false)
        if (typeof window !== "undefined") {
          localStorage.removeItem("isAuthenticated")
        }
        if (!pathName.startsWith("/auth")) router.push("/auth/login")
      }
    })()
  }, [pathName, router])

  // If we know the user is authenticated or is marked authenticated in localStorage
  if (isAuthenticated || isAuthSetInLocalStorage) {
    return <AppLayout>{children}</AppLayout>
  }
  // Otherwise, just render the children (loading states, or unauthenticated routes)
  return <main className="h-screen w-full flex flex-col">{children}</main>
}

export const useLogout = () => {
  const router = useRouter()

  const logout = useCallback(async () => {
    await logoutUser()
    if (typeof window !== "undefined") {
      localStorage.removeItem("isAuthenticated")
    }
    router.push("/")
  }, [router])

  return { logout }
}
