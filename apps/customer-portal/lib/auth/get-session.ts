import { cookies } from "next/headers"
import { Session } from "@ory/kratos-client"

import { authService } from "."

export const getSession = async (): Promise<{ data: Session; cookie: string } | null> => {
  const cookieParam = cookies()
    .getAll()
    .reduce((acc, cookie) => `${acc}${cookie.name}=${cookie.value}; `, "")

  const response = await authService().toSession({
    cookie: cookieParam,
  })

  if (response instanceof Error) {
    return null
  }

  return { data: response.data, cookie: cookieParam }
}
