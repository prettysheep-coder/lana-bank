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
import { Input } from "@/components/primitive/input"
import { Label } from "@/components/primitive/label"
import { Select } from "@/components/primitive/select"

export default function CreateLoanPage() {
  return (
    <main className="max-w-[70rem] m-auto mt-10">
      <Card className="flex-col h-full">
        <CardHeader>
          <div className="flex align-middle gap-4">
            <div>
              <LoanIcon className="hidden md:block w-10 h-10" />
            </div>
            <div className="flex flex-col gap-2">
              <CardTitle className="mt-2">Start a new Loan</CardTitle>
              <CardDescription>Fill Details to initiate a new loan</CardDescription>
            </div>
          </div>
        </CardHeader>
        <CardContent className="ml-14 flex flex-row gap-8">
          <div className="flex flex-col">
            <div className="flex flex-row gap-8">
              <div className="flex flex-col gap-6 w-60">
                <div>
                  <Label className="font-bold">USD Loan Amount</Label>
                  <Select>
                    <option value="" selected>
                      $100k USD
                    </option>
                    <option value="option1">$200k USD</option>
                    <option value="option2">$250k USD</option>
                    <option value="option3">$300k USD</option>
                  </Select>
                </div>
                <div>
                  <Label className="font-bold">Interest Rate</Label>
                  <p>5% fixed APR</p>
                </div>
                <div>
                  <Label className="font-bold">Duration</Label>
                  <p>6 months</p>
                </div>
              </div>
              <div className="flex flex-col gap-6">
                <div>
                  <Label className="font-bold">BTC Collateral Required</Label>
                  <p className="mt-2">2.38021243 BTC</p>
                </div>
                <div>
                  <Label className="font-bold">Collateral Value to Loan (CVL)</Label>
                  <p>150%</p>
                </div>
                <div>
                  <Label className="font-bold">Origination Fee</Label>
                  <p>1%</p>
                </div>
              </div>
            </div>
            <CardFooter className="gap-2 flex flex-col items-start pl-0 mt-8">
              <p className="text-sm">
                *Available BTC Balance: 0.00000000 BTC (Deposit BTC)
              </p>
              <Link href="/loan/create/approve" className="flex justify-start">
                <Button>Deposit BTC</Button>
              </Link>
            </CardFooter>
          </div>
          <Card variant="secondary" className="w-1/2">
            <CardHeader>
              <CardTitle>Total Cost Breakdown</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="list-disc pl-5">
                <li>Total Loan Amount: $100,000</li>
                <li>Total Interest Cost: $2,500</li>
                <li>Total Amount Repaid: $102,500</li>
                <li>Origination Fee: $1,000</li>
                <li>Total Cost of Loan: $105,155.32</li>
              </ul>
            </CardContent>
            <CardContent>
              <p className="text-sm mb-2">Collateral Value to Loan Details</p>
              <ul className="list-disc pl-5">
                <li>Target CVL: 150%</li>
                <li>Margin Call: 120%</li>
                <li>Loan Liquidation: 105%</li>
              </ul>
            </CardContent>
          </Card>
        </CardContent>
      </Card>
    </main>
  )
}
