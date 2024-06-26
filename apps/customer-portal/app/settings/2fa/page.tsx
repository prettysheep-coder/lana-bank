"use client"
import React from "react"

import { Button } from "@/components/primitive/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/primitive/card"
import { getTotpSecret } from "@/lib/settings/server-actions/initiate-totp-service"
import { validateTotpCode } from "@/lib/settings/server-actions/validate-totp-code"
import { SetupAuthenticatorDialog } from "@/components/settings/authenticator"
import { initiateWebauthnSetup } from "@/lib/settings/server-actions/initiate-webauthn-setup"
import { signupWithPasskey } from "@/lib/auth/utils/webauth"
import { validateWebAuth } from "@/lib/settings/server-actions/validate-webauth"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/primitive/dialog"
import { Input } from "@/components/primitive/input"

const SettingsPage = () => {
  const [totpSecret, setTotpSecret] = React.useState<string | null>(null)
  const [totpCode, setTotpCode] = React.useState<string>("")
  const [flowId, setFlowId] = React.useState<string | null>(null)
  const [error, setError] = React.useState<string | null>(null)
  const [csrfToken, setCsrfToken] = React.useState<string>("")
  const [openTotpDialog, setOpenTotpDialog] = React.useState<boolean>(false)
  const [webAuthPasskeyName, setWebAuthPasskeyName] = React.useState<string>("")
  const [openNameWebAuthnDialog, setOpenNameWebAuthnDialog] =
    React.useState<boolean>(false)
  const [webAuthData, setWebAuthData] = React.useState<{
    webauthnRegister: string
    flowId: string
    csrfToken: string
  } | null>(null)

  const handleTotpSetup = async () => {
    const response = await getTotpSecret()
    if (response.error) {
      setError(response.error.message)
      return
    }

    if (!response.data.totpSecret || !response.data.flowId || !response.data.csrfToken)
      return

    setFlowId(response.data.flowId)
    setTotpSecret(response.data.totpSecret)
    setCsrfToken(response.data.csrfToken)
    setOpenTotpDialog(true)
  }

  const handleSubmitTotp = async () => {
    if (!flowId || !totpCode) {
      return
    }

    const response = await validateTotpCode({
      flowId: flowId,
      totpCode: totpCode,
      csrfToken,
    })

    if (response.error) {
      setError(response.error.message)
      return
    }

    if (response.data.success) {
      setOpenTotpDialog(false)
    }
  }

  const handlePassKeySetup = async () => {
    const initiateWebauthnSetupResponse = await initiateWebauthnSetup()

    if (
      initiateWebauthnSetupResponse &&
      initiateWebauthnSetupResponse.data &&
      initiateWebauthnSetupResponse.data.webauthnRegisterTrigger
    ) {
      try {
        const { publicKey } = JSON.parse(
          initiateWebauthnSetupResponse.data.webauthnRegisterTrigger.slice(33, -1),
        )
        const signupWithPasskeyResponse = await signupWithPasskey(publicKey)

        if (!signupWithPasskeyResponse) {
          setError("Error Adding passkey")
        }

        setWebAuthData({
          webauthnRegister: signupWithPasskeyResponse,
          flowId: initiateWebauthnSetupResponse.data.flowId,
          csrfToken: initiateWebauthnSetupResponse.data.csrfToken,
        })

        setOpenNameWebAuthnDialog(true)
      } catch (error) {
        if (error instanceof Error) {
          setError(error.message)
        }
      }
    }
  }

  const validateWebAuthnHandler = async () => {
    if (!webAuthData) {
      return
    }

    const validateWebAuthResponse = await validateWebAuth({
      webauthnRegister: webAuthData?.webauthnRegister,
      flowId: webAuthData?.flowId,
      csrfToken: webAuthData?.csrfToken,
      webauthnRegisterDisplayname: webAuthPasskeyName,
    })

    if (validateWebAuthResponse.error) {
      setError(validateWebAuthResponse.error.message)
      return
    }

    if (validateWebAuthResponse.data.success) {
      setOpenNameWebAuthnDialog(false)
    }
  }

  return (
    <main className="max-w-[75rem] m-auto">
      <Card className="mt-24">
        <CardHeader>
          <CardTitle>Setup Two Factor Authentication</CardTitle>
          <CardDescription>Choose a method for securing your account.</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-row justify-between">
          <div className="flex flex-col ">
            <Button
              className=" text-left items-start justify-start "
              variant="ghost"
              onClick={handleTotpSetup}
            >
              Setup Authenticator
            </Button>
            <Button
              className=" text-left items-start justify-start "
              variant="ghost"
              onClick={handlePassKeySetup}
            >
              Setup PassKey
            </Button>
          </div>
        </CardContent>
      </Card>

      <Dialog
        open={openNameWebAuthnDialog}
        onOpenChange={() => {
          setError(null)
          setOpenNameWebAuthnDialog(!openNameWebAuthnDialog)
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Name Your Passkey</DialogTitle>
            <DialogDescription>
              This passkey will be identified by the name you assign. You can rename or
              remove it at any time in the future.
            </DialogDescription>
          </DialogHeader>
          <Input
            value={webAuthPasskeyName}
            onChange={(e) => setWebAuthPasskeyName(e.target.value)}
            placeholder="Enter a name for this passkey"
          />
          <DialogFooter>
            <Button onClick={validateWebAuthnHandler} variant="primary">
              Add Passkey
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {totpSecret && (
        <SetupAuthenticatorDialog
          openTotpDialog={openTotpDialog}
          setOpenTotpDialog={setOpenTotpDialog}
          totpSecret={totpSecret}
          setTotpCode={setTotpCode}
          totpCode={totpCode}
          setError={setError}
          handleSubmitTotp={handleSubmitTotp}
          error={error}
        />
      )}
    </main>
  )
}

export default SettingsPage
