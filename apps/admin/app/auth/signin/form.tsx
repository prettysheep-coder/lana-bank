import { getCsrfToken } from "next-auth/react"
import { cookies } from "next/headers"

import Input from "@/components/input"
import Button from "@/components/button"
import { BASE_PATH } from "@/app/config"

const SignInForm = async () => {
  const csrfToken = await getCsrfToken({
    req: {
      headers: {
        // Hack to get next-auth custom page working with App router and CSRF
        cookie: cookies().toString(),
      },
    },
  })

  return (
    <form
      className="mt-4 flex flex-col space-y-4 items-start"
      action={`${BASE_PATH}/api/auth/signin/email`}
      method="POST"
    >
      <input type="hidden" name="csrfToken" defaultValue={csrfToken} />
      <input type="hidden" name="callbackUrl" defaultValue={`${BASE_PATH}`} />
      <Input
        label="Email"
        type="email"
        name="email"
        placeholder="Please enter your email address"
        autofocus
        required
      />
      <Button type="submit" title="Submit" />
      <p className="text-default">
        By continuing, you consent to our cookie policy, terms of service and privacy
        policy
      </p>
    </form>
  )
}

export default SignInForm
