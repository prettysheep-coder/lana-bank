import * as React from "react"

import { ReactNode } from "react"

import { Card, CardHeader, CardTitle, CardDescription } from "@lana/web/ui/card"
import { DetailItemProps, DetailsCard } from "@lana/web/components/details"

import { meQuery } from "@/lib/graphql/query/me"

import { Badge } from "@lana/web/ui/badge"

import { AccountStatus } from "@/lib/graphql/generated"

import { formatDate } from "@/lib/utils"

import { CustomerTransactionsTable } from "./transaction"

export default async function Home() {
  const data = await meQuery()
  if (data instanceof Error) {
    return <div>{data.message}</div>
  }

  const customer = data.me?.customer

  const details: DetailItemProps[] = [
    {
      label: "Email",
      value: customer.email,
    },
    { label: "Onboarded on", value: formatDate(customer.createdAt) },
    {
      label: "Telegram",
      value: customer.telegramId,
    },
    {
      label: "Status",
      value: (
        <Badge
          variant={customer.status === AccountStatus.Active ? "success" : "secondary"}
        >
          {customer.status}
        </Badge>
      ),
    },
  ]

  const transactions = [
    ...(customer?.depositAccount.deposits || []),
    ...(customer?.depositAccount.withdrawals || []),
  ].sort((a, b) => {
    return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
  })

  return (
    <main className="max-w-7xl mx-auto px-2 flex flex-col gap-2">
      <DetailsCard
        title={<div className="text-md font-medium">Welcome Back!</div>}
        details={details}
      />
      <div className="flex flex-col md:flex-row gap-2">
        <BalanceCard
          title="Pending Balance"
          description="Funds in process, not available yet"
          h1={data.me?.customer.depositAccount.balance.pending}
        />
        <BalanceCard
          title="Settled Balance"
          description="Funds ready to use or withdraw"
          h1={data.me?.customer.depositAccount.balance.settled}
        />
      </div>
      <CustomerTransactionsTable transactions={transactions} />
    </main>
  )
}

type BalanceCardProps = {
  h1?: ReactNode
  title: string
  description: string
}

const BalanceCard: React.FC<BalanceCardProps> = ({ h1, title, description }) => {
  return (
    <Card className="w-full" data-testid={title.toLowerCase().replace(" ", "-")}>
      <CardHeader>
        <CardDescription className="text-md font-medium">{title}</CardDescription>
        <div className="flex flex-col">
          <CardTitle className="text-4xl">{h1}</CardTitle>
        </div>
        <CardDescription className="text-sm">{description}</CardDescription>
      </CardHeader>
    </Card>
  )
}
