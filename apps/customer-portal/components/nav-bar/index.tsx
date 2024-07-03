import { headers } from "next/headers"

import { AuthenticatorAssuranceLevel } from "@ory/client"

import { NavBarAuthenticated } from "./nav-bar-authenticated"

import { verifyToken } from "@/lib/auth/jwks"
import { getSession } from "@/lib/auth/get-session.ts"

export default async function NavBar() {
  const token = headers().get("authorization")
  if (!token) return null //TODO: maybe add a navbar for unauthenticated users ?

  const decodedToken = await verifyToken(token.split(" ")[1])
  if (decodedToken.sub === "anonymous") return null

  const session = await getSession()
  if (session instanceof Error) return null

  const email = session.userData?.email
  if (!email) return null

  return (
    <NavBarAuthenticated
      email={email}
      twoFactorEnabled={
        session.kratosSession.authenticator_assurance_level ===
        AuthenticatorAssuranceLevel.Aal2
      }
    />
  )
}
