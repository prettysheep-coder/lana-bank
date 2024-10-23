import type { Metadata } from "next"

import SignInForm from "./form"

import Icon from "@/components/icon"

const SignIn: React.FC = () => {
  return (
    <div className="p-4 flex flex-col justify-center h-full -mt-20">
      <Icon />
      <h1 className="mt-10 text-headline">Sign In</h1>
      <h2 className="mt-4 text-title">Welcome to Lava Bank Admin Panel</h2>
      <h3 className="text-description">Enter your email address to continue</h3>
      <SignInForm />
    </div>
  )
}

export default SignIn

export const metadata: Metadata = {
  title: "Sign In | Lava Bank Admin",
}
