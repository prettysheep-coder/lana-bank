"use server"

import { UiNodeInputAttributes, UiNodeTextAttributes } from "@ory/kratos-client"

import { initiateSettingsFlow } from "../api/initiate-settings-flow"

import { getSession } from "@/lib/auth/get-session"
import { createErrorResponse } from "@/lib/utils"

interface ExtendedUiNodeTextAttributes extends UiNodeTextAttributes {
  name?: string
}

export const initiateWebauthnSetup = async (): Promise<
  ServerActionResponse<{
    webauthnRegisterTrigger: string
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
  const flowId = res.data.id

  const webauthnRegisterTriggerNode = res.data.ui.nodes.find(
    (node) =>
      (node.attributes as ExtendedUiNodeTextAttributes)?.name ===
      "webauthn_register_trigger",
  )
  let webauthnRegisterTrigger: string | undefined
  if (
    webauthnRegisterTriggerNode &&
    "onclick" in webauthnRegisterTriggerNode.attributes
  ) {
    webauthnRegisterTrigger = (
      webauthnRegisterTriggerNode.attributes as UiNodeInputAttributes
    ).onclick
  }

  const csrfNode = res.data.ui.nodes.find(
    (node) => (node.attributes as ExtendedUiNodeTextAttributes)?.name === "csrf_token",
  )
  let csrfToken: string | undefined
  if (csrfNode && "value" in csrfNode.attributes) {
    csrfToken = (csrfNode.attributes as UiNodeInputAttributes).value
  }

  if (!webauthnRegisterTrigger || !csrfToken)
    return createErrorResponse({ errorMessage: "Attributes not found" })

  return {
    data: { webauthnRegisterTrigger, flowId, csrfToken },
    error: null,
  }
}
