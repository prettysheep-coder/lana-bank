"use client"

import { useEffect, useState, useCallback } from "react"
import { usePathname, useRouter } from "next/navigation"

import { getSession, logoutUser } from "./ory"

type Props = {
  appChildren: React.ReactNode
  children: React.ReactNode
}

export const Authenticated: React.FC<Props> = ({ appChildren, children }) => {
  const router = useRouter()
  const pathName = usePathname()

  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null)
  useEffect(() => {
    ;(async () => {
      try {
        await getSession()
        setIsAuthenticated(true)
        if (pathName === "/") router.push("/dashboard")
      } catch (error) {
        setIsAuthenticated(false)
        if (!pathName.startsWith("/auth")) router.push("/auth/login")
      }
    })()
  }, [pathName, router])

  if (!isAuthenticated)
    return <main className="h-screen w-full flex flex-col">{appChildren}</main>
  else return <>{children}</>
}

export const useLogout = () => {
  const router = useRouter()

  const logout = useCallback(async () => {
    await logoutUser()
    router.push("/")
  }, [router])

  return { logout }
}
