"use server"

import { cookies } from "next/headers"

import { redirect } from "next/navigation"

import { authService } from ".."

import { setCookieFromString } from "./set-cookie-from-string"

export const createLoginOrRegisterFlow = async ({
  email,
}: {
  email: string
}): Promise<void | {
  response: null
  errorMessage: string | null
}> => {
  const startSignInFlow = await authService().startSignInFlow({ email })
  if (startSignInFlow instanceof Error) {
    return console.error("Failed to start sign in flow", startSignInFlow)
  }

  if (startSignInFlow.messageId === 4000035) {
    console.error("user does not exist start register flow")

    const startRegisterFlow = await authService().startRegisterFlow({
      email,
    })

    if (startRegisterFlow instanceof Error) {
      return {
        response: null,
        errorMessage: startRegisterFlow.message,
      }
    }

    startRegisterFlow.responseCookies.forEach(setCookieFromString)
    cookies().set({
      name: "csrfToken",
      value: startRegisterFlow.csrfToken,
      httpOnly: true,
      sameSite: "lax",
      secure: true,
    })

    redirect("/auth/register/otp?flow=" + startRegisterFlow.flowId)
  }

  startSignInFlow.responseCookies.forEach(setCookieFromString)
  cookies().set({
    name: "csrfToken",
    value: startSignInFlow.csrfToken,
    httpOnly: true,
    sameSite: "lax",
    secure: true,
  })
  redirect("/auth/signin/otp?flow=" + startSignInFlow.flowId)
}
