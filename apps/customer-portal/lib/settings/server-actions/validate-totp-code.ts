"use server"

import { AxiosError } from "axios"

import { validateTotp } from "../api/validate-totp"

import { getSession } from "@/lib/auth/get-session"
import { createErrorResponse } from "@/lib/utils"

export const validateTotpCode = async ({
  totpCode,
  flowId,
  csrfToken,
}: {
  totpCode: string
  flowId: string
  csrfToken: string
}): Promise<
  ServerActionResponse<{
    success: boolean
  }>
> => {
  const session = await getSession()
  const sessionCookie = session?.cookie
  if (!session || !sessionCookie)
    return createErrorResponse({ errorMessage: "Session not found" })

  const res = await validateTotp({
    cookie: sessionCookie,
    totpCode,
    flowId,
    csrfToken,
  })

  if (res instanceof AxiosError) {
    if (
      res.response?.data?.ui?.messages[0]?.id &&
      res.response?.data?.ui?.messages[0]?.text
    ) {
      return createErrorResponse({
        errorMessage: res.response?.data.ui.messages[0].text,
        id: res.response?.data.ui.messages[0].id,
      })
    }

    return createErrorResponse({
      errorMessage: "Something went wrong, please try again.",
    })
  }

  if (res instanceof Error) return createErrorResponse({ errorMessage: res.message })

  return {
    data: { success: true },
    error: null,
  }
}
