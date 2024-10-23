import type { Metadata } from "next"

import Icon from "@/components/icon"

const Verify: React.FC = () => {
  return (
    <div className="p-4 flex flex-col justify-center h-full -mt-20">
      <Icon />
      <h1 className="mt-10 text-headline">A verification email has been sent</h1>
      <h2 className="mt-4 text-title">Welcome to Lava Bank Admin Panel</h2>
      <p className="mt-4 text-description">
        Click the link in your email address to continue. We&apos;ve sent a confirmation
        email to your inbox. Please check your email to proceed.
      </p>
    </div>
  )
}

export default Verify

export const metadata: Metadata = {
  title: "Verify | Lava Bank Admin",
}
