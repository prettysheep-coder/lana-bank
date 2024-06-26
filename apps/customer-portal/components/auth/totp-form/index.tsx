"use client"
import { useState } from "react"

import { useRouter } from "next/navigation"

import { AxiosError } from "axios"

import { Button } from "@/components/primitive/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/primitive/card"
import { Input } from "@/components/primitive/input"
import { Alert, AlertDescription } from "@/components/primitive/alert"
import { submitTotpFow } from "@/lib/kratos/public/submit-totp"

const TotpForm = ({ flowId }: { flowId: string }) => {
  const router = useRouter()
  const [totpCode, setTotpCode] = useState("")
  const [error, setError] = useState<string | null>(null)

  const handleTotpSubmit = async (event: React.FormEvent) => {
    event.preventDefault()
    setError(null)
    if (!totpCode) {
      return
    }

    try {
      await submitTotpFow({
        flowId,
        totpCode,
      })
      router.push("/")
    } catch (error) {
      console.error(error)
      if (error instanceof AxiosError) {
        if (
          error.response?.data?.ui?.messages[0]?.id &&
          error.response?.data?.ui?.messages[0]?.text
        ) {
          setError(error.response?.data.ui.messages[0].text)
          return
        }
      }
      setError("Something went wrong. Please try again.")
    }
  }

  return (
    <Card variant="transparent" className="md:w-2/5">
      <CardHeader className="pt-4">
        <CardTitle>Authenticator Code</CardTitle>
        <CardDescription className="text-textColor-secondary">
          Please enter your authenticator code to continue.
        </CardDescription>
      </CardHeader>
      <form onSubmit={handleTotpSubmit}>
        <CardContent>
          <Input
            type="text"
            value={totpCode}
            onChange={(e) => setTotpCode(e.target.value)}
            placeholder="Please enter code"
          />
        </CardContent>
        <CardFooter className="flex flex-col gap-2">
          <Button type="submit" className="rounded-full px-6 w-full">
            Next
          </Button>
          {error && (
            <Alert variant="destructive" className="mt-1 p-3">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}
        </CardFooter>
      </form>
    </Card>
  )
}

export { TotpForm }
