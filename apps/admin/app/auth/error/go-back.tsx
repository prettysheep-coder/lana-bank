"use client"

import { useRouter } from "next/navigation"

import Button from "@/components/button"
import { BASE_PATH } from "@/app/config"

export const GoBack = () => {
  const router = useRouter()

  return (
    <Button
      className="mt-8"
      onClick={() => {
        router.push(`${BASE_PATH}/auth/signin`)
      }}
      title="Try Again"
    />
  )
}
