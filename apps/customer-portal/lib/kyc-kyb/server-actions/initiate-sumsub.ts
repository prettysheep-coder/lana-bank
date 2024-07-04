"use server"

import { sumSubTokenCreate } from "@/lib/graphql/mutation/sumsub-token-create"

export const initiateKycKyb = async (): Promise<
  ServerActionResponse<{
    token: string
  }>
> => {
  const sumSubTokenCreateResponse = await sumSubTokenCreate()

  if (sumSubTokenCreateResponse instanceof Error) {
    return {
      data: null,
      error: {
        message: sumSubTokenCreateResponse.message,
      },
    }
  }

  return {
    data: {
      token: sumSubTokenCreateResponse.sumsubTokenCreate.token,
    },
    error: null,
  }
}
