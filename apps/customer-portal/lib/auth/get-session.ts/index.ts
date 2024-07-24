import { cookies } from "next/headers"

import { Session } from "@ory/client"

import { meQuery } from "@/lib/graphql/query/me"
import { toSession } from "@/lib/kratos/public/to-session"
import { MeQuery } from "@/lib/graphql/generated"

export const getMeAndSession = async (): Promise<
  | {
      me: MeQuery["me"]
      session: Session
    }
  | Error
> => {
  const meQueryResponse = await meQuery()

  console.log({ meQueryResponse })

  if (meQueryResponse instanceof Error) {
    return meQueryResponse
  }

  const cookieParam = cookies()
    .getAll()
    .reduce((acc, cookie) => `${acc}${cookie.name}=${cookie.value}; `, "")

  console.log({ cookieParam })

  try {
    const kratosSession = await toSession({
      cookie: cookieParam,
    })

    console.log({ kratosSession })

    if (kratosSession instanceof Error) {
      console.error("Error getting session from Kratos: ", kratosSession)
      return kratosSession
    }

    return {
      me: meQueryResponse.me,
      session: kratosSession,
    }
  } catch (error) {
    console.log("Error getting session from Kratos: ", error)
    return new Error("Error getting session from Kratos")
  }
}
