import CredentialsProvider from "next-auth/providers/credentials"

import { CallbacksOptions } from "next-auth"

const providers = []

providers.push(
  CredentialsProvider({
    name: "Credentials",
    credentials: {
      username: { label: "Username", type: "text" },
      password: { label: "Password", type: "password" },
    },
    authorize: async (credentials) => {
      if (credentials?.username === "admin" && credentials?.password === "admin") {
        return { id: "1", name: "admin", email: "admin@galoy.io" }
      }
      if (credentials?.username === "user" && credentials?.password === "user") {
        return { id: "2", name: "user", email: "user@galoy.io" }
      }
      return null
    },
  }),
)

const callbacks: Partial<CallbacksOptions> = {
  // ignore ts error
  // @ts-ignore
  async signIn({ account, profile, user }) {
    if (
      account?.provider === "credentials"
      // && env.NODE_ENV === "development"
    ) {
      return !!user
    }

    if (!account || !profile) {
      return false
    }

    const email = profile?.email
    if (!email) {
      return false
    }

    // eslint-disable-next-line no-new-wrappers
    const verified = new Boolean("email_verified" in profile && profile.email_verified)
    return verified // && env.AUTHORIZED_EMAILS.includes(email)
  },
}

export const authOptions = {
  providers,
  callbacks,
}
