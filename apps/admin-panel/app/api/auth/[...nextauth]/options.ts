import EmailProvider from "next-auth/providers/email"
import PostgresAdapter from "@auth/pg-adapter"

import { Pool } from "pg"
import { NextAuthOptions } from "next-auth"
import { Adapter } from "next-auth/adapters"

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

const pool = new Pool({
  connectionString: process.env.NEXT_AUTH_DATABASE_URL,
})

export const authOptions: NextAuthOptions = {
  providers: [
    EmailProvider({
      server: process.env.EMAIL_SERVER,
      from: process.env.EMAIL_FROM,
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
  adapter: PostgresAdapter(pool) as Adapter,
}
