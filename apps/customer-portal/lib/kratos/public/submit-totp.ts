import { AxiosError } from "axios"

import { kratosPublic } from "@/lib/kratos/sdk"

import { getCsrfToken } from "@/lib/kratos/utils"

type SubmitTotpData = {
  flowId: string
  totpCode: string
}

export const submitTotpFow = async ({ flowId, totpCode }: SubmitTotpData) => {
  const method = "totp"
  const flow = await kratosPublic.getLoginFlow({ id: flowId })

  if (flow instanceof AxiosError) return flow

  const csrfToken = getCsrfToken(flow.data)
  if (!csrfToken) throw new Error("Kratos API didn't send CSRF token")

  const data = await kratosPublic.updateLoginFlow({
    flow: flowId,
    updateLoginFlowBody: {
      method,
      totp_code: totpCode,
      csrf_token: csrfToken,
    },
  })

  return data
}
