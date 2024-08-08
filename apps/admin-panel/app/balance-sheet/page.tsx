"use client"
import { gql } from "@apollo/client"
import { useState } from "react"

import { Account } from "./account"

import {
  BalanceSheetQuery,
  StatementCategoryWithBalance,
  useBalanceSheetQuery,
} from "@/lib/graphql/generated"
import Balance, { Currency } from "@/components/balance/balance"
import {
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableRow,
} from "@/components/primitive/table"

import { PageHeading } from "@/components/page-heading"
import { Select } from "@/components/primitive/select"

gql`
  query BalanceSheet {
    balanceSheet {
      name
      balance {
        ...balancesByCurrency
      }
      categories {
        name
        balance {
          ...balancesByCurrency
        }
        accounts {
          ... on AccountWithBalance {
            __typename
            id
            name
            balance {
              ...balancesByCurrency
            }
          }
          ... on AccountSetWithBalance {
            __typename
            id
            name
            hasSubAccounts
            balance {
              ...balancesByCurrency
            }
          }
        }
      }
    }
  }
`
const BALANCE_FOR_CATEGORY: {
  [key: string]: { TransactionType: TransactionType }
} = {
  Liabilities: { TransactionType: "netCredit" },
  Equity: { TransactionType: "netCredit" },
  Assets: { TransactionType: "netDebit" },
}

export default function BalanceSheetPage() {
  const {
    data: BalanceSheetData,
    loading: BalanceSheetLoading,
    error: BalanceSheetError,
  } = useBalanceSheetQuery()

  return (
    <BalanceSheet
      data={BalanceSheetData?.balanceSheet}
      loading={BalanceSheetLoading}
      error={BalanceSheetError}
    />
  )
}

const BalanceSheet = ({
  data,
  loading,
  error,
}: {
  data: BalanceSheetQuery["balanceSheet"]
  loading: boolean
  error: Error | undefined
}) => {
  const [currency, setCurrency] = useState<Currency>("btc")
  const [layer, setLayer] = useState<Layers>("all")

  const balance = data?.balance
  const categories = data?.categories

  if (error) return <div className="text-destructive">{error.message}</div>
  if (loading) return <div>Loading...</div>
  if (!balance) return <div>No data</div>

  return (
    <main>
      <div className="flex justify-between">
        <PageHeading>Balance Sheet</PageHeading>
        <div className="flex gap-4">
          <div className="w-32">
            <Select
              value={currency}
              onChange={(e) => setCurrency(e.target.value as Currency)}
            >
              <option value="btc">BTC</option>
              <option value="usd">USD</option>
            </Select>
          </div>
          <div className="w-32">
            <Select value={layer} onChange={(e) => setLayer(e.target.value as Layers)}>
              <option value="all">All</option>
              <option value="settled">Settled</option>
              <option value="pending">Pending</option>
              <option value="encumbrance">Encumbrance</option>
            </Select>
          </div>
        </div>
      </div>
      <Table>
        <TableBody>
          {categories?.map((category) => {
            console.log(
              category.name,
              BALANCE_FOR_CATEGORY[category.name].TransactionType,
            )

            return (
              <CategoryRow
                key={category.name}
                category={category}
                currency={currency}
                layer={layer}
                transactionType={
                  BALANCE_FOR_CATEGORY[category.name].TransactionType || "netCredit"
                }
              />
            )
          })}
        </TableBody>
        <TableFooter>
          <TableRow className="border-t-4">
            <TableCell className="uppercase font-bold  text-textColor-secondary ">
              Net
            </TableCell>
            <TableCell className="w-48 flex flex-col gap-2 items-end text-right">
              <Balance currency={currency} amount={balance[currency][layer].netDebit} />
            </TableCell>
          </TableRow>
        </TableFooter>
      </Table>
    </main>
  )
}

const CategoryRow = ({
  category,
  currency,
  layer,
  transactionType,
}: {
  category: StatementCategoryWithBalance
  currency: Currency
  layer: Layers
  transactionType: TransactionType
}) => {
  return (
    <>
      <TableRow>
        <TableCell className="flex items-center gap-2 text-primary font-semibold uppercase">
          {category.name}
        </TableCell>
        <TableCell className="w-48">
          <Balance
            currency={currency}
            amount={category.balance[currency][layer][transactionType]}
          />
        </TableCell>
      </TableRow>
      {category.accounts.map((account) => (
        <Account
          key={account.id}
          account={account}
          currency={currency}
          layer={layer}
          transactionType={transactionType}
        />
      ))}
    </>
  )
}
