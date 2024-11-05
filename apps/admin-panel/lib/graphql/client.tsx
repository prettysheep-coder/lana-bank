"use client"

import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client"
import createUploadLink from "apollo-upload-client/createUploadLink.mjs"
import { relayStylePagination } from "@apollo/client/utilities"

import { env } from "@/env"

const uploadLink = createUploadLink({
  uri: env.NEXT_PUBLIC_CORE_ADMIN_URL,
  credentials: "include",
})

const cache = new InMemoryCache({
  typePolicies: {
    AccountSetAndSubAccounts: {
      fields: {
        subAccounts: relayStylePagination(),
      },
    },
    Query: {
      fields: {
        customers: relayStylePagination(),
        loans: relayStylePagination(),
        creditFacilities: relayStylePagination(),
      },
    },
  },
})

const client = new ApolloClient({
  cache,
  link: uploadLink,
})

const GQLClient: React.FC<React.PropsWithChildren> = ({ children }) => (
  <ApolloProvider client={client}>{children}</ApolloProvider>
)

export default GQLClient
