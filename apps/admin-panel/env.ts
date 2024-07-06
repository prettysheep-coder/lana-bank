import { createEnv } from "@t3-oss/env-nextjs"
import { z } from "zod"

export const env = createEnv({
  shared: {
    NEXT_PUBLIC_CORE_ADMIN_URL: z
      .string()
      .url()
      .default("http://localhost:4455/admin/graphql"),
    NEXTAUTH_SECRET: z.string().default("secret"),
  },
  runtimeEnv: {
    NEXT_PUBLIC_CORE_ADMIN_URL: process.env.NEXT_PUBLIC_CORE_ADMIN_URL,
    NEXTAUTH_SECRET: process.env.NEXTAUTH_SECRET,
  },
})
