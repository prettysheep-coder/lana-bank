"use client"

import { gql } from "@apollo/client"
import { Button } from "@lana/web/ui/button"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@lana/web/ui/dialog"
import { Label } from "@lana/web/ui/label"
import { Input } from "@lana/web/ui/input"
import { useTranslations } from "next-intl"
import { useEffect, useState } from "react"

import {
  DepositModuleConfig,
  DepositModuleConfigUpdateInput,
  useDepositConfigUpdateMutation,
} from "@/lib/graphql/generated"

gql`
  mutation depositConfigUpdate($input: DepositModuleConfigUpdateInput!) {
    depositConfigUpdate(input: $input) {
      depositConfig {
        chartOfAccountsId
        chartOfAccountsDepositAccountsParentCode
        chartOfAccountsOmnibusParentCode
      }
    }
  }
`

type DepositConfigUpdateDialogProps = {
  setOpen: (isOpen: boolean) => void
  open: boolean
  depositModuleConfig?: DepositModuleConfig
}

export const DepositConfigUpdateDialog: React.FC<DepositConfigUpdateDialogProps> = ({
  open,
  setOpen,
  depositModuleConfig,
}) => {
  const t = useTranslations("Modules")
  const tCommon = useTranslations("Common")

  const [
    updateDepositConfig,
    { loading: updateDepositConfigLoading, error: updateDepositConfigError },
  ] = useDepositConfigUpdateMutation()
  const [formData, setFormData] = useState<DepositModuleConfigUpdateInput>({
    chartOfAccountsId: "",
    chartOfAccountsDepositAccountsParentCode: "",
    chartOfAccountsOmnibusParentCode: "",
  })

  useEffect(() => {
    if (
      depositModuleConfig &&
      depositModuleConfig.chartOfAccountsId &&
      depositModuleConfig.chartOfAccountsDepositAccountsParentCode &&
      depositModuleConfig.chartOfAccountsOmnibusParentCode
    ) {
      setFormData({
        chartOfAccountsId: depositModuleConfig.chartOfAccountsId,
        chartOfAccountsDepositAccountsParentCode:
          depositModuleConfig.chartOfAccountsDepositAccountsParentCode,
        chartOfAccountsOmnibusParentCode:
          depositModuleConfig.chartOfAccountsOmnibusParentCode,
      })
    }
  }, [depositModuleConfig])

  return (
    <Dialog open={open} onOpenChange={() => setOpen(false)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("deposit.setTitle")}</DialogTitle>
        </DialogHeader>
        <Label>{t("deposit.chartOfAccountsId")}</Label>
        <Input
          value={formData.chartOfAccountsId}
          onChange={(e) =>
            setFormData({ ...formData, chartOfAccountsId: e.target.value })
          }
        />
        <Label>{t("deposit.chartOfAccountsDepositAccountsParentCode")}</Label>
        <Input
          value={formData.chartOfAccountsDepositAccountsParentCode}
          onChange={(e) =>
            setFormData({
              ...formData,
              chartOfAccountsDepositAccountsParentCode: e.target.value,
            })
          }
        />
        <Label>{t("deposit.chartOfAccountsOmnibusParentCode")}</Label>
        <Input
          value={formData.chartOfAccountsOmnibusParentCode}
          onChange={(e) =>
            setFormData({ ...formData, chartOfAccountsOmnibusParentCode: e.target.value })
          }
        />
        {updateDepositConfigError && (
          <div className="text-destructive">{updateDepositConfigError.message}</div>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={() => setOpen(false)}>
            {tCommon("cancel")}
          </Button>
          <Button
            loading={updateDepositConfigLoading}
            onClick={async () => {
              await updateDepositConfig({ variables: { input: formData } })
              setOpen(false)
            }}
          >
            {tCommon("save")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
