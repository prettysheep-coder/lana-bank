import { SuccessfulNativeLogin } from "@ory/kratos-client"
import { AxiosError, AxiosResponse } from "axios"

import { kratosPublic } from "../../kratos-sdk"

export const verifyTotpCodeFlow = async ({
  flow,
  totpCode,
  cookie,
  csrfToken,
}: {
  flow: string
  totpCode: string
  csrfToken: string
  cookie: string
}): Promise<Error | AxiosError | AxiosResponse<SuccessfulNativeLogin>> => {
  const method = "totp"
  try {
    const res = await kratosPublic.updateLoginFlow({
      flow,
      cookie,
      updateLoginFlowBody: {
        method,
        csrf_token: csrfToken,
        totp_code: totpCode,
      },
    })

    return res
  } catch (error) {
    if (error instanceof AxiosError) {
      return error as AxiosError
    }
    return new Error("Something went wrong, please try again.")
  }
}
