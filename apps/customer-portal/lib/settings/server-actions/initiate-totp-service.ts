"use server"

import { UiNodeInputAttributes, UiNodeTextAttributes } from "@ory/kratos-client"

import { initiateSettingsFlow } from "../api/initiate-settings-flow"

import { getSession } from "@/lib/auth/get-session"
import { createErrorResponse } from "@/lib/utils"

interface ExtendedUiNodeTextAttributes extends UiNodeTextAttributes {
  name?: string
}

export const getTotpSecret = async (): Promise<
  ServerActionResponse<{
    totpSecret: string
    flowId: string
    csrfToken: string
  }>
> => {
  const session = await getSession()
  const sessionCookie = session?.cookie

  if (!session || !sessionCookie)
    return createErrorResponse({ errorMessage: "Session not found" })

  const res = await initiateSettingsFlow(sessionCookie)

  if (res instanceof Error) return createErrorResponse({ errorMessage: res.message })

  const totpAttributes = res.data.ui.nodes.find(
    (node) => (node.attributes as UiNodeTextAttributes).id === "totp_secret_key",
  )

  const csrfNode = res.data.ui.nodes.find(
    (node) => (node.attributes as ExtendedUiNodeTextAttributes)?.name === "csrf_token",
  )

  if (!totpAttributes)
    return createErrorResponse({ errorMessage: "TOTP secret not found" })

  const totpSecret = (totpAttributes.attributes as UiNodeTextAttributes).text.text
  const flowId = res.data.id

  let csrfToken = ""
  if (csrfNode && "value" in csrfNode.attributes) {
    csrfToken = (csrfNode.attributes as UiNodeInputAttributes).value
  }

  return {
    data: { totpSecret, flowId, csrfToken },
    error: null,
  }
}
