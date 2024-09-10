import React from "react"

import { GetLoanDetailsQuery } from "@/lib/graphql/generated"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/primitive/table"
import { formatDate } from "@/lib/utils"
import Balance from "@/components/balance/balance"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/primitive/card"

type RepaymentPlanProps = {
  loan: NonNullable<GetLoanDetailsQuery["loan"]>
}
export const RepaymentPlan: React.FC<RepaymentPlanProps> = ({ loan }) => {
  return (
    <Card className="mt-4">
      <CardHeader>
        <CardTitle>Repayment Plan</CardTitle>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Repayment Type</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Initial</TableHead>
              <TableHead>Outstanding</TableHead>
              <TableHead>Accural At</TableHead>
              <TableHead>Due At</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loan.repaymentPlan.map((r, i) => (
              <TableRow key={i}>
                <TableCell>{r.repaymentType}</TableCell>
                <TableCell>{r.status}</TableCell>
                <TableCell>
                  <Balance amount={r.initial} currency="usd" />
                </TableCell>
                <TableCell>
                  <Balance amount={r.outstanding} currency="usd" />
                </TableCell>
                <TableCell>{formatDate(r.accrualAt)}</TableCell>
                <TableCell>{formatDate(r.dueAt)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}
