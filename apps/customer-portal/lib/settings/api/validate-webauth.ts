import { AxiosError, AxiosResponse } from "axios"

import { SettingsFlow } from "@ory/kratos-client"

import { kratosPublic } from "@/lib/kratos-sdk"

export const validateWebauthSettings = async ({
  cookie,
  webauthnRegister,
  flowId,
  csrfToken,
  webauthnRegisterDisplayname,
}: {
  cookie: string
  webauthnRegister: string
  flowId: string
  csrfToken: string
  webauthnRegisterDisplayname: string
}): Promise<AxiosResponse<SettingsFlow> | AxiosError | Error> => {
  const method = "webauthn"
  try {
    return await kratosPublic.updateSettingsFlow({
      flow: flowId,
      cookie,
      updateSettingsFlowBody: {
        method,
        csrf_token: csrfToken,
        webauthn_register: webauthnRegister,
        webauthn_register_displayname: webauthnRegisterDisplayname,
      },
    })
  } catch (error) {
    if (error instanceof AxiosError) {
      return error
    }
    return new Error("Something went wrong, please try again.")
  }
}
