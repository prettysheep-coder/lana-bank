import EmailProvider from "next-auth/providers/email"
import CredentialsProvider from "next-auth/providers/credentials"
import { NextAuthOptions } from "next-auth"
import axios from "axios"

import { customPostgresAdapter } from "@/lib/auth/db/auth-adapter"
import { pool } from "@/lib/auth/db"
import { basePath, env } from "@/env"

async function checkUserEmail(email: string): Promise<boolean> {
  try {
    const response = await axios.post(env.CHECK_USER_ALLOWED_CALLBACK_URL, {
      email,
      transient_payload: {},
    })

    console.log("User check response:", response.status)
    return response.status === 200
  } catch (error) {
    console.error("Error checking user:", error)
    return false
  }
}

const isPreviewEnvironment = process.env.VERCEL_ENV === "preview"

export const authOptions: NextAuthOptions = {
  providers: [
    ...(!isPreviewEnvironment
      ? [
          EmailProvider({
            server: env.EMAIL_SERVER,
            from: env.EMAIL_FROM,
          }),
        ]
      : []),

    ...(isPreviewEnvironment
      ? [
          CredentialsProvider({
            name: "Preview Access",
            credentials: {
              email: { label: "Email", type: "email" },
              password: { label: "Password", type: "password" },
            },
            async authorize(credentials) {
              if (!credentials?.email || !credentials?.password) return null

              if (
                credentials.email === "galoysuperuser@mailinator.com" &&
                credentials.password === "admin"
              ) {
                return {
                  id: "preview-user",
                  email: credentials.email,
                  name: "Preview User",
                }
              }
              return null
            },
          }),
        ]
      : []),
  ],

  session: {
    strategy: "jwt",
  },

  callbacks: {
    async redirect() {
      return `${basePath}/dashboard`
    },
    async signIn({ account, credentials }) {
      if (isPreviewEnvironment && credentials) {
        return true
      }

      if (account?.provider === "email" && account.providerAccountId) {
        return checkUserEmail(account.providerAccountId)
      }

      return false
    },
    async session({ session, token }) {
      if (session.user && token.email) {
        session.user.name = token.email.split("@")[0]
        session.user.email = token.email
      }
      return session
    },
  },

  adapter: isPreviewEnvironment ? undefined : customPostgresAdapter(pool),
  secret: env.NEXTAUTH_SECRET,

  ...(isPreviewEnvironment
    ? {}
    : {
        pages: {
          signIn: `${basePath}/auth/login`,
          error: `${basePath}/auth/error`,
          verifyRequest: `${basePath}/auth/verify`,
        },
      }),
}
