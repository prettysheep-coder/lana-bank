import EmailProvider from "next-auth/providers/email"
import PostgresAdapter from "@auth/pg-adapter"

import { Pool } from "pg"
import { NextAuthOptions } from "next-auth"
import { Adapter } from "next-auth/adapters"

const allowedEmails = ["admin@lava.io", "user@lava.io"]

const pool = new Pool({
  connectionString: process.env.NEXT_AUTH_DATABASE_URL,
})

const providers = []

providers.push(
  EmailProvider({
    server: process.env.EMAIL_SERVER,
    from: process.env.EMAIL_FROM,
  }),
)

export const authOptions: NextAuthOptions = {
  providers,
  session: {
    strategy: "jwt",
  },
  callbacks: {
    async signIn({ account }) {
      const email = account?.providerAccountId
      if (account?.provider === "email" && email && allowedEmails.includes(email)) {
        return true
      }
      return false
    },
  },
  adapter: PostgresAdapter(pool) as Adapter,
}
