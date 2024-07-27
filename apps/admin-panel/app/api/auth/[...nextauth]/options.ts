import EmailProvider from "next-auth/providers/email"

import { NextAuthOptions } from "next-auth"

import { customPostgresAdapter } from "@/lib/auth/db/auth-adapter"
import { pool } from "@/lib/auth/db"
import { env } from "@/env"

const allowedUsers = [
  {
    id: 1,
    name: "Admin",
    email: "admin@lava.io",
    role: "admin",
  },
  {
    id: 2,
    name: "User",
    email: "user@lava.io",
    role: "user",
  },
]

export const authOptions: NextAuthOptions = {
  providers: [
    EmailProvider({
      server: env.EMAIL_SERVER,
      from: env.EMAIL_FROM,
    }),
  ],
  session: {
    strategy: "jwt",
  },
  callbacks: {
    async signIn({ account }) {
      const email = account?.providerAccountId
      const user = allowedUsers.find((user) => user.email === email)
      if (account?.provider === "email" && email && user) {
        return true
      }
      return false
    },
    async session({ session, token }) {
      const user = allowedUsers.find((allowedUser) => allowedUser.email === token.email)
      if (session.user) {
        session.user.name = user?.name
        session.user.email = token.email
      }
      return session
    },
  },
  adapter: customPostgresAdapter(pool),
}
