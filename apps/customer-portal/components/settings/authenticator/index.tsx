import QRCode from "react-qr-code"

import { useState } from "react"

import { AxiosError } from "axios"

import { Button } from "@/components/primitive/button"
import { CopyButton } from "@/components/primitive/copy-button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/primitive/dialog"
import { Input } from "@/components/primitive/input"
import {
  createTotpSetupFlow,
  submitTotpSetupFlow,
} from "@/lib/kratos/public/setup-totp-flow"
import { AttributesNotFoundError } from "@/lib/kratos/error"

const SetupAuthenticator = () => {
  const [totpCode, setTotpCode] = useState<string>("")
  const [error, setError] = useState<string | null>(null)
  const [openTotpDialog, setOpenTotpDialog] = useState<boolean>(false)

  const [flowData, setFlowData] = useState<{
    flowId: string
    totpSecret: string
    csrfToken: string
  } | null>(null)

  const handleTotpSetup = async () => {
    const createTotpSetupFlowResponse = await createTotpSetupFlow()

    if (createTotpSetupFlowResponse instanceof AxiosError) {
      setError(createTotpSetupFlowResponse.message)
      return
    }

    if (createTotpSetupFlowResponse instanceof AttributesNotFoundError) {
      setError(createTotpSetupFlowResponse.message)
      return
    }

    const { flowId, totpSecret, csrfToken } = createTotpSetupFlowResponse
    setFlowData({
      flowId,
      totpSecret,
      csrfToken,
    })
    setOpenTotpDialog(true)
  }

  const handleSubmitTotp = async () => {
    if (!flowData || !totpCode) {
      return
    }

    try {
      const submitTotpSetupFlowResponse = await submitTotpSetupFlow({
        flowId: flowData.flowId,
        totpCode: totpCode,
        csrfToken: flowData.csrfToken,
      })

      if (submitTotpSetupFlowResponse.success) {
        setOpenTotpDialog(false)
      }
    } catch (error) {
      console.log(error)
    }
  }

  return (
    <>
      <Button onClick={handleTotpSetup}>Setup Authenticator App</Button>
      {flowData && (
        <Dialog
          open={openTotpDialog}
          onOpenChange={() => {
            setError(null)
            setTotpCode("")
            setOpenTotpDialog(!openTotpDialog)
          }}
        >
          <DialogContent className="max-w-96">
            <DialogHeader className="flex flex-col space-y-1.5 text-center">
              <DialogTitle className="text-center">Setup Authenticator</DialogTitle>
              <DialogDescription className="text-center">
                Scan the QR code with your authenticator app and enter the code below
              </DialogDescription>
            </DialogHeader>
            <div className="flex flex-col justify-center items-center gap-4">
              <div className="flex justify-center items-center bg-white p-4 rounded-lg ">
                <QRCode size={200} value={flowData.totpSecret || ""} />
              </div>
              <div className="bg-secondary-foreground p-1 rounded-md px-2 flex gap-2 items-center">
                <p className="text-textColor-secondary text-xs">{flowData.totpSecret}</p>
                <CopyButton value={flowData.totpSecret} />
              </div>
              <Input
                value={totpCode}
                onChange={(e) => setTotpCode(e.target.value)}
                placeholder="Enter the code from your authenticator app"
              />
              <Button className="w-full" onClick={handleSubmitTotp}>
                Submit
              </Button>
              {error && <p className="text-red-500">{error}</p>}
            </div>
          </DialogContent>
        </Dialog>
      )}
    </>
  )
}

export { SetupAuthenticator }
