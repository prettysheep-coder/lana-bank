import { gql } from "@apollo/client"

import { MeDocument, MeQuery, MeQueryVariables } from "../generated"

import { executeQuery } from "."

gql`
  query me {
    me {
      customer {
        id
        customerId
      }
    }
  }
`

export const meQuery = async () => {
  return executeQuery<MeQuery, MeQueryVariables>({
    document: MeDocument,
    variables: {},
  })
}
