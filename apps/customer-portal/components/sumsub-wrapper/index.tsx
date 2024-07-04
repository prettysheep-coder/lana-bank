"use client"
import React, { useState } from "react"
import SumsubWebSdk from "@sumsub/websdk-react"

import { toast } from "sonner"

import { Checkbox } from "../primitive/check-box"
import { Label } from "../primitive/label"

import { Dialog, DialogContent } from "../primitive/dialog"

import { initiateKycKyb } from "@/lib/kyc-kyb/server-actions/initiate-sumsub"

function KycKybWrapper({ kycCompleted }: { kycCompleted: boolean }) {
  const [token, setToken] = useState<null | string>(null)
  const [isDialogOpen, setIsDialogOpen] = useState(false)

  const initiateKycKybHandler = async () => {
    if (kycCompleted) return toast.info("KYC or KYB onboarding already completed")

    const initiateKycKybResponse = await initiateKycKyb()
    if (initiateKycKybResponse.error) {
      toast.error(initiateKycKybResponse.error.message)
      return
    }

    setToken(initiateKycKybResponse.data.token)
    setIsDialogOpen(true)
  }

  return (
    <>
      <div onClick={initiateKycKybHandler} className="flex gap-2 items-center">
        <Checkbox checked={kycCompleted} />
        <Label className="hover:underline">Complete KYC or KYB onboarding</Label>
      </div>
      {token && (
        <Dialog
          open={isDialogOpen}
          onOpenChange={() => {
            setIsDialogOpen(false)
            setToken(null)
          }}
        >
          <DialogContent className="p-8">
            <SumsubWebSdk
              onError={(error: string) => {
                toast.error(error)
              }}
              expirationHandler={() => {
                toast.error("KYC or KYB onboarding expired")
              }}
              accessToken={token}
            />
          </DialogContent>
        </Dialog>
      )}
    </>
  )
}

export { KycKybWrapper }
