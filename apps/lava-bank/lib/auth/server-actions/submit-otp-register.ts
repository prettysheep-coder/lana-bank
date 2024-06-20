"use server"
import { cookies } from "next/headers"

import { AxiosError } from "axios"

import { redirect } from "next/navigation"

import { authService } from ".."
import { getCsrfCookiesAsString } from "../utils"

import { setCookieFromString } from "./set-cookie-from-string"

export const submitOtpRegister = async ({
  flowId,
  code,
  email,
}: {
  flowId: string
  code: string
  email: string
}): Promise<void | {
  response: null
  errorMessage: string | null
}> => {
  const csrfToken = cookies().get("csrfToken")?.value
  const allCookies = cookies().getAll()

  if (!csrfToken)
    return {
      response: null,
      errorMessage: "not able to find csrf token",
    }

  const res = await authService().verifyEmailCodeRegisterFlow({
    code,
    csrfToken,
    email,
    flow: flowId,
    cookie: getCsrfCookiesAsString(allCookies),
  })

  if (res instanceof AxiosError) {
    if (
      res.response?.data?.ui?.messages[0]?.id &&
      res.response?.data?.ui?.messages[0]?.text
    ) {
      return {
        response: null,
        errorMessage: res.response?.data.ui.messages[0].text,
      }
    }

    return {
      response: null,
      errorMessage: "Something went wrong, please try again.",
    }
  }

  if (res instanceof Error) {
    return {
      response: null,
      errorMessage: res.message,
    }
  }

  if (!res.headers["set-cookie"]) {
    return {
      response: null,
      errorMessage: "Something went wrong, please try again.",
    }
  }

  res.headers["set-cookie"].forEach(setCookieFromString)

  if (res.status === 200) {
    redirect("/")
  }
}
