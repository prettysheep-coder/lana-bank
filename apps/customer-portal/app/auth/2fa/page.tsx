import { redirect } from "next/navigation"

import { cookies } from "next/headers"

import { AuthTemplateCard } from "@/components/auth/auth-template-card"
import { TotpForm } from "@/components/auth/totp-form"
import { authService } from "@/lib/auth"
import { getCsrfCookiesAsString } from "@/lib/auth/utils"

async function TwoFactorAuthPage({
  searchParams,
}: {
  searchParams: {
    flow?: string
  }
}) {
  const flowId = searchParams?.flow
  const allCookies = cookies().getAll()

  if (!flowId) {
    redirect("/auth")
  }

  const signInFlow = await authService().getLoginFlow({
    flowId,
    cookie: getCsrfCookiesAsString(allCookies),
  })

  if (signInFlow instanceof Error) {
    redirect("/auth")
  }

  return (
    <AuthTemplateCard>
      <TotpForm flowId={flowId} />
    </AuthTemplateCard>
  )
}
export default TwoFactorAuthPage
