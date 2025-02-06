"use client"

import React from "react"

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@lana/web/ui/card"

import DataTable, { Column } from "@lana/web/components/data-table"

import { Badge, BadgeProps } from "@lana/web/ui/badge"

import { MeQuery, WithdrawalStatus } from "@/lib/graphql/generated"

import { formatDate } from "@/lib/utils"

type Deposit = NonNullable<
  MeQuery["me"]["customer"]
>["depositAccount"]["deposits"][number]
type Withdrawal = NonNullable<
  MeQuery["me"]["customer"]
>["depositAccount"]["withdrawals"][number]
type Transaction = Deposit | Withdrawal

const isWithdrawal = (transaction: Transaction): transaction is Withdrawal => {
  return "withdrawalId" in transaction
}

type CustomerTransactionsTableProps = {
  transactions: Transaction[]
}

export const CustomerTransactionsTable: React.FC<CustomerTransactionsTableProps> = ({
  transactions,
}) => {
  const columns: Column<Transaction>[] = [
    {
      key: "createdAt",
      header: "Date",
      render: (value: string) => formatDate(value),
    },
    {
      key: "reference",
      header: "Type",
      render: (_: string, record: Transaction) =>
        isWithdrawal(record) ? "Withdrawal" : "Deposit",
    },
    {
      key: "amount",
      header: "Amount",
      render: (value) => value,
    },
    {
      key: "reference",
      header: "Status",
      render: (_: string, record: Transaction) =>
        isWithdrawal(record) ? <WithdrawalStatusBadge status={record.status} /> : "N/A",
    },
  ] as const

  const getNavigateUrl = (record: Transaction) => {
    if (isWithdrawal(record)) {
      return `/withdrawals/${record.withdrawalId}`
    }
    return null
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Transactions</CardTitle>
        <CardDescription>Your Past Withdraw and Deposits</CardDescription>
      </CardHeader>
      <CardContent>
        <DataTable
          data={transactions}
          columns={columns}
          emptyMessage={
            <div className="min-h-[10rem] w-full border rounded-md flex items-center justify-center">
              No transactions found
            </div>
          }
          navigateTo={getNavigateUrl}
          className="w-full table-fixed"
          headerClassName="bg-secondary [&_tr:hover]:!bg-secondary"
        />
      </CardContent>
    </Card>
  )
}

interface StatusBadgeProps extends BadgeProps {
  status: WithdrawalStatus
}

const getVariant = (status: WithdrawalStatus) => {
  switch (status) {
    case WithdrawalStatus.PendingApproval:
      return "default"
    case WithdrawalStatus.Confirmed:
      return "success"
    case WithdrawalStatus.Cancelled:
      return "destructive"
    case WithdrawalStatus.Denied:
      return "destructive"
    default:
      return "default"
  }
}

export const WithdrawalStatusBadge: React.FC<StatusBadgeProps> = ({ status }) => {
  const variant = getVariant(status)
  return <Badge variant={variant}>{status.split("_").join(" ")}</Badge>
}
