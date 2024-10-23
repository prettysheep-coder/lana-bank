import type { Metadata } from "next"
import { getCsrfToken } from "next-auth/react"

import Input from "@/components/input"
import Button from "@/components/button"
import Icon from "@/components/icon"
import { BASE_PATH } from "@/app/config"

const SignIn: React.FC = async () => {
  const csrfToken = await getCsrfToken()
  return (
    <div className="p-4 flex flex-col justify-center h-full -mt-20">
      <Icon />
      <h1 className="mt-10 text-headline">Sign In</h1>
      <h2 className="mt-4 text-title">Welcome to Lava Bank Admin Panel</h2>
      <h3 className="text-description">Enter your email address to continue</h3>
      <form
        className="mt-4 flex flex-col space-y-4 items-start"
        action={`${BASE_PATH}/api/auth/signin/email`}
        method="POST"
      >
        <input name="csrfToken" type="hidden" defaultValue={csrfToken} />
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
    </div>
  )
}

export default SignIn

export const metadata: Metadata = {
  title: "Sign In | Lava Bank Admin",
}
