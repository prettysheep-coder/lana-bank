import React from "react"

import { GetLoanDetailsQuery } from "@/lib/graphql/generated"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/primitive/card"
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

type RepaymentPlanProps = {
  loan: NonNullable<GetLoanDetailsQuery["loan"]>
}
const RepaymentPlan: React.FC<RepaymentPlanProps> = ({ loan }) => {
  return (
    <Card className="mt-4">
      <CardHeader>
        <CardTitle>Repayment Plan</CardTitle>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Type</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Accural At</TableHead>
              <TableHead>Due At</TableHead>
              <TableHead>
                <span className="flex justify-end">Initial</span>
              </TableHead>
              <TableHead>
                <span className="flex justify-end">Outstanding</span>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loan.repaymentPlan.map((repayment, index) => (
              <TableRow key={index}>
                <TableCell>{repayment.repaymentType}</TableCell>
                <TableCell>{repayment.status}</TableCell>
                <TableCell>{formatDate(repayment.accrualAt)}</TableCell>
                <TableCell>{formatDate(repayment.dueAt)}</TableCell>
                <TableCell>
                  <Balance
                    className="flex justify-end"
                    amount={repayment.initial}
                    currency="usd"
                  />
                </TableCell>
                <TableCell>
                  <Balance
                    className="flex justify-end"
                    amount={repayment.outstanding}
                    currency="usd"
                  />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}

export default RepaymentPlan
