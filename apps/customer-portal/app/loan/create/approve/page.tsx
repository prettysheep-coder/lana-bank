"use client"
import Link from "next/link"

import { LoanIcon } from "@/components/icons"
import { Button } from "@/components/primitive/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/primitive/card"
import { Checkbox } from "@/components/primitive/check-box"
import { Label } from "@/components/primitive/label"

export default function CreateLoanPage() {
  return (
    <main className="max-w-[70rem] m-auto mt-10">
      <Card className="flex-col h-full">
        <CardHeader>
          <div className="flex align-middle gap-4">
            <LoanIcon className="hidden md:block w-10 h-10" />
            <div className="flex flex-col gap-2">
              <CardTitle className="mt-2">Start a new Loan</CardTitle>
              <CardDescription>Review details and sign loan contract.</CardDescription>
            </div>
          </div>
        </CardHeader>
        <CardContent className="ml-8 flex flex-col md:flex-row justify-between">
          <div className="w-1/2">
            <Card variant="transparent">
              <CardHeader>
                <CardTitle>Loan Details</CardTitle>
              </CardHeader>
              <CardContent className="flex flex-col text-sm">
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">USD Loan</p>
                  <p className="font-semibold">$100,000</p>
                </div>
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">Collateral Value to Loan</p>
                  <p className="font-semibold">$100,000</p>
                </div>
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">Fees</p>
                  <p className="font-semibold">$100,000</p>
                </div>
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">Duration</p>
                  <p className="font-semibold">$100,000</p>
                </div>
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">Interest</p>
                  <p className="font-semibold">$100,000</p>
                </div>
              </CardContent>
            </Card>
            <Card variant="transparent">
              <CardHeader>
                <CardTitle>Collateral Details</CardTitle>
              </CardHeader>
              <CardContent className="flex flex-col text-sm">
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">Collateral required</p>
                  <p className="font-semibold">2.38021243 BTC</p>
                </div>
                <div className="flex justify-between hover:bg-secondary-foreground p-0.5 px-2 rounded-md">
                  <p className="text-textColor-secondary">BTC Account Balance</p>
                  <p className="font-semibold">0.00000000 BTC</p>
                </div>
              </CardContent>
              <CardFooter className="gap-4 flex flex-col items-start mt-4">
                <div className="flex gap-2 items-center">
                  <Checkbox />
                  <Label>Agree to full terms and conditions</Label>
                </div>
                <div className="flex gap-2 items-center">
                  <Checkbox />
                  <Label>Pledge collateral from my Lava Bank BTC Account</Label>
                </div>
                <div className="flex gap-2 items-center align-middle">
                  <Link href="/loan/create/approve" className="flex justify-start mt-4">
                    <Button>Deposit BTC</Button>
                  </Link>
                  <p className="mt-4 ml-4">or edit loan details</p>
                </div>
              </CardFooter>
            </Card>
          </div>
          <div className="w-2/3">
            <Card variant="secondary">
              <CardHeader>
                <CardTitle>Loan Contract Terms</CardTitle>
                <CardDescription>
                  Please review the following terms and details carefully before
                  initiating your new loan.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <ul className="list-disc pl-5">
                  <li>Interest Rate: 5% (fixed for 6 month term)</li>
                  <li>
                    Interest Accrual: Monthly Payment Schedule: Full repayment upon term
                    end
                  </li>
                  <li>
                    Early Repayment Penalty: None Loan Disbursement Time: Within 24 to 48
                  </li>
                  <li>hours after approval</li>
                </ul>
              </CardContent>
              <CardContent>
                <p className="text-textColor-secondary text-sm mb-2">
                  Collateral Value to Loan (CVL) Details.
                </p>
                <ul className="list-disc pl-5">
                  <li>Target CVL: 150%</li>
                  <li>Margin Call: 120%</li>
                  <li>Loan Liquidation: 105%</li>
                </ul>
              </CardContent>
              <CardContent>
                Once you have read and agreed to the loan contract, click Sign & Initiate
                Loan to complete the loan process.
              </CardContent>
            </Card>
          </div>
        </CardContent>
      </Card>
    </main>
  )
}
