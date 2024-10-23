import type { Metadata } from "next"

import { GoBack } from "./go-back"

import Icon from "@/components/icon"

type ErrorSearchParams = {
  error?: string
}

type ErrorProps = {
  searchParams?: ErrorSearchParams
}

const Error: React.FC<ErrorProps> = ({ searchParams }) => {
  return (
    <div className="p-4 flex flex-col justify-center h-full -mt-20">
      <Icon />
      <h1 className="mt-10 text-headline">{searchParams?.error}</h1>
      <h2 className="mt-4 text-title">Welcome to Lava Bank Admin Panel</h2>
      <h3 className="text-description">Oops, we could not sign you in</h3>
      <div className="mt-4 text-title text-error">
        Please recheck your credentials and try again. Repeated attempts with wrong email
        might ban your IP from the system.
      </div>
      <GoBack />
    </div>
  )
}

export default Error

export const metadata: Metadata = {
  title: "Denied | Lava Bank Admin",
}
