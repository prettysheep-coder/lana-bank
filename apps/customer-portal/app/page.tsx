import Link from "next/link"

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/primitive/card"

import { DownloadIcon, RocketIcon } from "@/components/icons"
import { Checkbox } from "@/components/primitive/check-box"
import { Label } from "@/components/primitive/label"
import { Button } from "@/components/primitive/button"

export default function Home() {
  return (
    <main className="max-w-[70rem] m-auto">
      <OnboardingCard twoFactorAuthEnabled={false} kycCompleted={false} />
      <div className="flex gap-4 mt-4 items-stretch">
        <BalanceCard />
        <LoanCard />
      </div>
    </main>
  )
}

const BalanceCard = () => {
  return (
    <Card className="w-1/3 flex-col h-full">
      <CardHeader>
        <CardTitle>Balance</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        <div className="flex justify-between">
          <p>Bitcoin</p>
          <p>0 BTC</p>
        </div>
        <div className="flex justify-between">
          <p>USD</p>
          <p>$0.00</p>
        </div>
      </CardContent>
      <CardFooter className="justify-center gap-2">
        <Button>Deposit BTC</Button>
        <Button variant="secondary">Withdraw</Button>
      </CardFooter>
    </Card>
  )
}

const LoanCard = () => {
  return (
    <Card className="w-2/3">
      <CardHeader>
        <CardTitle className="flex justify-between align-middle items-center">
          Loans
          <Link href="/loan/create">
            <Button>New Loan</Button>
          </Link>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <Card variant="secondary">
          <CardHeader className="flex">
            <div className="flex align-middle gap-4">
              <DownloadIcon className="w-10 h-10 text-primary" />
              <div className="flex flex-col gap-2">
                <CardTitle>Add funds to start a loan</CardTitle>
                <CardDescription>
                  Curious how much to deposit? Explore loan options and rates
                </CardDescription>
              </div>
            </div>
          </CardHeader>
        </Card>
      </CardContent>
    </Card>
  )
}

const OnboardingCard = ({
  twoFactorAuthEnabled,
  kycCompleted,
}: {
  twoFactorAuthEnabled: boolean
  kycCompleted: boolean
}) => {
  return (
    <Card className="mt-10">
      <CardHeader className="md:pb-0">
        <div className="flex align-middle gap-4">
          <RocketIcon className="hidden md:block w-10 h-10" />
          <div className="flex flex-col gap-2">
            <CardTitle className="mt-2">
              Complete onboarding steps to Initiate a Loan
            </CardTitle>
            <CardDescription>
              Complete the following steps to initiate to complete your onboarding process
            </CardDescription>
          </div>
        </div>
      </CardHeader>
      <CardContent className="mt-6">
        <div className="ml-14 flex flex-col gap-4">
          <Link className="flex gap-2 items-center" href="/settings/2fa">
            <Checkbox checked={twoFactorAuthEnabled} />
            <Label className="hover:underline">Enable Two-Factor Authentication </Label>
          </Link>
          <Link className="flex gap-2 items-center" aria-disabled href="/settings">
            <Checkbox checked={kycCompleted} />
            <Label className="hover:underline">Complete KYC or KYB onboarding</Label>
          </Link>
        </div>
      </CardContent>
    </Card>
  )
}
