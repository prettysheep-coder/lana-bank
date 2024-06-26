"use server"
import { cookies } from "next/headers"

import { AxiosError } from "axios"

import { redirect } from "next/navigation"

import { authService } from ".."

import { createErrorResponse } from "@/lib/utils"

export const submitTotpCode = async ({
  flowId,
  totpCode,
}: {
  flowId: string
  totpCode: string
}): Promise<void | ServerActionResponse<null>> => {
  const csrfToken = cookies().get("csrfToken")?.value
  const allCookies = cookies().getAll()

  const cookieParam = allCookies.reduce(
    (acc, cookie) => `${acc}${cookie.name}=${cookie.value}; `,
    "",
  )

  if (!csrfToken)
    return createErrorResponse({
      errorMessage: "Something went wrong, please try again.",
    })

  const res = await authService().verifyTotpCodeFlow({
    totpCode,
    csrfToken,
    flow: flowId,
    cookie: cookieParam,
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

  if (!res.data.session?.identity?.id)
    return createErrorResponse({
      errorMessage: "No identity found",
    })

  if (res.status === 200) redirect("/")
}
