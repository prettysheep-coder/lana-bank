import { AxiosError } from "axios"

import { kratosPublic } from "@/lib/kratos-sdk"

export const initiateSettingsFlow = async (sessionCookie: string) => {
  try {
    return await kratosPublic.createBrowserSettingsFlow({ cookie: sessionCookie })
  } catch (error) {
    if (error instanceof AxiosError) {
      return error
    }
    return new Error("An error occurred while initiating TOTP")
  }
}
