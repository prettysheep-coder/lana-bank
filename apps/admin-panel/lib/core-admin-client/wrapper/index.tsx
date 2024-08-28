"use client"

import { ApolloLink, HttpLink } from "@apollo/client"
import { relayStylePagination } from "@apollo/client/utilities"
import {
  ApolloClient,
  ApolloNextAppProvider,
  InMemoryCache,
  SSRMultipartLink,
} from "@apollo/experimental-nextjs-app-support"

import {
  GetRealtimePriceUpdatesDocument,
  GetRealtimePriceUpdatesQuery,
  Loan,
} from "@/lib/graphql/generated"

function makeClient({ coreAdminGqlUrl }: { coreAdminGqlUrl: string }) {
  const httpLink = new HttpLink({
    uri: coreAdminGqlUrl,
    fetchOptions: { cache: "no-store" },
  })

  return new ApolloClient({
    defaultOptions: {
      query: {
        fetchPolicy: "no-cache",
      },
      watchQuery: {
        fetchPolicy: "no-cache",
      },
    },
    cache: new InMemoryCache({
      typePolicies: {
        AccountSetAndSubAccounts: {
          fields: {
            subAccounts: relayStylePagination(),
          },
        },
        Query: {
          fields: {
            loans: relayStylePagination(),
          },
        },
      },
    }),
    resolvers: {
      Loan: {
        currentCvl: async (loan: Loan, _, { cache }) => {
          const fetchData = () =>
            new Promise((resolve) => {
              const priceInfo = cache.readQuery({
                query: GetRealtimePriceUpdatesDocument,
              }) as GetRealtimePriceUpdatesQuery

              if (priceInfo) {
                resolve(priceInfo)
              } else {
                setTimeout(() => resolve(fetchData()), 500) // Try again after 500 ms
              }
            })

          const priceInfo = (await fetchData()) as GetRealtimePriceUpdatesQuery

          const collateral_value_in_sats = loan.balance.collateral.btcBalance
          const collateral_value_in_cents =
            (priceInfo.realtimePrice.usdCentsPerBtc * collateral_value_in_sats) /
            100_000_000
          const collateral_value_in_usd = collateral_value_in_cents / 100

          const outstanding_amount_in_usd = loan.balance.outstanding.usdBalance / 100

          if (collateral_value_in_usd == 0 || outstanding_amount_in_usd == 0) return 0

          const cvl = (collateral_value_in_usd / outstanding_amount_in_usd) * 100

          return cvl.toFixed(2)
        },
      },
    },
    link:
      typeof window === "undefined"
        ? ApolloLink.from([
            new SSRMultipartLink({
              stripDefer: true,
            }),
            httpLink,
          ])
        : httpLink,
  })
}

export default function ApolloWrapper({
  config,
  children,
}: {
  config: {
    coreAdminGqlUrl: string
  }
  children: React.ReactNode
}) {
  const client = makeClient(config)
  return (
    <ApolloNextAppProvider makeClient={() => client}>{children}</ApolloNextAppProvider>
  )
}
