import { AxiosError } from "axios"

import { AuthenticatorAssuranceLevel } from "@ory/kratos-client"

import { kratosPublic } from "../../kratos-sdk"

import { getCsrfToken } from "@/lib/utils"

export const startAal2SignInFlow = async ({
  cookie,
}: {
  cookie: string
}): Promise<
  | Error
  | AxiosError
  | {
      flowId: string
      csrfToken: string
      responseCookies: string[]
    }
> => {
  try {
    const signInFlow = await kratosPublic.createBrowserLoginFlow({
      aal: AuthenticatorAssuranceLevel.Aal2,
      cookie,
      refresh: true,
    })
    const csrfToken = getCsrfToken(signInFlow.data)
    const responseCookies = signInFlow.headers["set-cookie"]
    const flowId = signInFlow.data.id

    if (!csrfToken || !responseCookies || !flowId) {
      return new Error(
        "attributes are missing, please try again, or contact support if error persist",
      )
    }

    return {
      flowId,
      csrfToken,
      responseCookies,
    }
  } catch (error) {
    if (error instanceof AxiosError) {
      return error
    }
    return new Error("An error occurred while initiating TOTP")
  }
}
