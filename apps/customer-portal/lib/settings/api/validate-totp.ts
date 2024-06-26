import { AxiosError, AxiosResponse } from "axios"

import { SettingsFlow } from "@ory/kratos-client"

import { kratosPublic } from "@/lib/kratos-sdk"

export const validateTotp = async ({
  cookie,
  totpCode,
  flowId,
  csrfToken,
}: {
  cookie: string
  totpCode: string
  flowId: string
  csrfToken: string
}): Promise<AxiosResponse<SettingsFlow> | AxiosError | Error> => {
  const method = "totp"
  try {
    return await kratosPublic.updateSettingsFlow({
      flow: flowId,
      cookie,
      updateSettingsFlowBody: {
        csrf_token: csrfToken,
        method,
        totp_code: totpCode,
      },
    })
  } catch (error) {
    if (error instanceof AxiosError) {
      return error
    }
    return new Error("Something went wrong, please try again.")
  }
}
