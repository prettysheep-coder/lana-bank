import { AxiosError } from "axios"
import { UiNodeInputAttributes, UiNodeTextAttributes } from "@ory/client"

import { kratosPublic } from "../sdk"
import { AttributesNotFoundError } from "../error"

interface ExtendedUiNodeTextAttributes extends UiNodeTextAttributes {
  name?: string
}

export const createTotpSetupFlow = async (): Promise<{
  totpSecret: string
  flowId: string
  csrfToken: string
}> => {
  try {
    const { data } = await kratosPublic.createBrowserSettingsFlow()

    const totpAttributes = data.ui.nodes.find(
      (node) => (node.attributes as UiNodeTextAttributes).id === "totp_secret_key",
    )

    if (!totpAttributes) {
      throw new AttributesNotFoundError("TOTP attribute not found.")
    }

    const totpSecret = (totpAttributes.attributes as UiNodeTextAttributes).text.text
    const flowId = data.id

    const csrfNode = data.ui.nodes.find(
      (node) => (node.attributes as ExtendedUiNodeTextAttributes)?.name === "csrf_token",
    )

    let csrfToken = ""
    if (csrfNode && "value" in csrfNode.attributes) {
      csrfToken = (csrfNode.attributes as UiNodeInputAttributes).value
    }

    return {
      totpSecret,
      flowId,
      csrfToken,
    }
  } catch (error) {
    if (error instanceof AxiosError) {
      throw new Error(
        error.response?.data?.ui?.messages[0]?.text ||
          "Something went wrong, please try again.",
      )
    }
    throw error
  }
}

export const submitTotpSetupFlow = async ({
  flowId,
  totpCode,
  csrfToken,
}: {
  flowId: string
  totpCode: string
  csrfToken: string
}): Promise<{ success: boolean }> => {
  try {
    await kratosPublic.updateSettingsFlow({
      flow: flowId,
      updateSettingsFlowBody: {
        csrf_token: csrfToken,
        method: "totp",
        totp_code: totpCode,
      },
    })

    return {
      success: true,
    }
  } catch (error) {
    if (error instanceof AxiosError) {
      throw new Error(
        error.response?.data?.ui?.messages[0]?.text ||
          "Something went wrong, please try again.",
      )
    }
    throw error
  }
}
