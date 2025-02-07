"use client"

import React from "react"
import { GetCreditFacilityQuery } from "@/lib/graphql/generated"
import { formatDate } from "@/lib/utils"
import DataTable, { Column } from "@lana/web/components/data-table"
import { Badge, BadgeProps } from "@lana/web/ui/badge"
import Balance from "@/components/balance"

type RepaymentPlan = NonNullable<
  NonNullable<GetCreditFacilityQuery["creditFacility"]>["repaymentPlan"][number]
>

type CreditFacilityRepaymentPlanProps = {
  creditFacility: NonNullable<GetCreditFacilityQuery["creditFacility"]>
}

export const CreditFacilityRepaymentPlan: React.FC<CreditFacilityRepaymentPlanProps> = ({
  creditFacility,
}) => {
  const columns: Column<RepaymentPlan>[] = [
    {
      key: "repaymentType",
      header: "Type",
      render: (type) => getRepaymentTypeDisplay(type),
    },
    {
      key: "initial",
      header: "Initial Amount",
      render: (amount) => <Balance amount={amount} currency="usd" />,
    },
    {
      key: "outstanding",
      header: "Outstanding",
      render: (amount) => <Balance amount={amount} currency="usd" />,
    },
    {
      key: "dueAt",
      header: "Due Date",
      render: (date) => formatDate(date),
    },
    {
      key: "status",
      header: "Status",
      align: "right",
      render: (_, repayment) => {
        return <RepaymentStatusBadge status={repayment.status} />
      },
    },
  ]

  const repaymentPlanData = creditFacility?.repaymentPlan ?? []

  return (
    <DataTable
      data={repaymentPlanData}
      columns={columns}
      emptyMessage="No repayment plan found"
    />
  )
}

interface StatusBadgeProps extends BadgeProps {
  status: RepaymentPlan["status"]
}

const getStatusVariant = (status: RepaymentPlan["status"]): BadgeProps["variant"] => {
  switch (status) {
    case "UPCOMING":
      return "default"
    case "DUE":
      return "warning"
    case "OVERDUE":
      return "destructive"
    case "PAID":
      return "success"
    default:
      return "default"
  }
}

const RepaymentStatusBadge: React.FC<StatusBadgeProps> = ({ status, ...props }) => {
  const variant = getStatusVariant(status)
  return (
    <Badge variant={variant} {...props}>
      {status}
    </Badge>
  )
}

const getRepaymentTypeDisplay = (type: RepaymentPlan["repaymentType"]) => {
  switch (type) {
    case "DISBURSAL":
      return "Principal"
    case "INTEREST":
      return "Interest"
    default:
      return type
  }
}
