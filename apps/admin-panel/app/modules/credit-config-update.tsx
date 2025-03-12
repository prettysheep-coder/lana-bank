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
  CreditModuleConfig,
  CreditModuleConfigUpdateInput,
  useCreditConfigUpdateMutation,
} from "@/lib/graphql/generated"

gql`
  mutation CreditConfigUpdate($input: CreditModuleConfigUpdateInput!) {
    creditConfigUpdate(input: $input) {
      creditConfig {
        chartOfAccountsId
        chartOfAccountFacilityOmnibusParentCode
        chartOfAccountCollateralOmnibusParentCode
        chartOfAccountFacilityParentCode
        chartOfAccountCollateralParentCode
        chartOfAccountDisbursedReceivableParentCode
        chartOfAccountInterestReceivableParentCode
        chartOfAccountInterestIncomeParentCode
        chartOfAccountFeeIncomeParentCode
      }
    }
  }
`

type CreditConfigUpdateDialogProps = {
  setOpen: (isOpen: boolean) => void
  open: boolean
  creditModuleConfig?: CreditModuleConfig
}

export const CreditConfigUpdateDialog: React.FC<CreditConfigUpdateDialogProps> = ({
  open,
  setOpen,
  creditModuleConfig,
}) => {
  const t = useTranslations("Modules")
  const tCommon = useTranslations("Common")

  const [
    updateCreditConfig,
    { loading: updateCreditConfigLoading, error: updateCreditConfigError },
  ] = useCreditConfigUpdateMutation()
  const [formData, setFormData] = useState<CreditModuleConfigUpdateInput>({
    chartOfAccountsId: "",
    chartOfAccountFacilityOmnibusParentCode: "",
    chartOfAccountCollateralOmnibusParentCode: "",
    chartOfAccountFacilityParentCode: "",
    chartOfAccountCollateralParentCode: "",
    chartOfAccountDisbursedReceivableParentCode: "",
    chartOfAccountInterestReceivableParentCode: "",
    chartOfAccountInterestIncomeParentCode: "",
    chartOfAccountFeeIncomeParentCode: "",
  })

  useEffect(() => {
    if (
      creditModuleConfig &&
      creditModuleConfig.chartOfAccountsId &&
      creditModuleConfig.chartOfAccountFacilityOmnibusParentCode &&
      creditModuleConfig.chartOfAccountCollateralOmnibusParentCode &&
      creditModuleConfig.chartOfAccountFacilityParentCode &&
      creditModuleConfig.chartOfAccountCollateralParentCode &&
      creditModuleConfig.chartOfAccountDisbursedReceivableParentCode &&
      creditModuleConfig.chartOfAccountInterestReceivableParentCode &&
      creditModuleConfig.chartOfAccountInterestIncomeParentCode &&
      creditModuleConfig.chartOfAccountFeeIncomeParentCode
    ) {
      setFormData({
        chartOfAccountsId: creditModuleConfig.chartOfAccountsId,
        chartOfAccountFacilityOmnibusParentCode:
          creditModuleConfig.chartOfAccountFacilityOmnibusParentCode,
        chartOfAccountCollateralOmnibusParentCode:
          creditModuleConfig.chartOfAccountCollateralOmnibusParentCode,
        chartOfAccountFacilityParentCode:
          creditModuleConfig.chartOfAccountFacilityParentCode,
        chartOfAccountCollateralParentCode:
          creditModuleConfig.chartOfAccountCollateralParentCode,
        chartOfAccountDisbursedReceivableParentCode:
          creditModuleConfig.chartOfAccountDisbursedReceivableParentCode,
        chartOfAccountInterestReceivableParentCode:
          creditModuleConfig.chartOfAccountInterestReceivableParentCode,
        chartOfAccountInterestIncomeParentCode:
          creditModuleConfig.chartOfAccountInterestIncomeParentCode,
        chartOfAccountFeeIncomeParentCode:
          creditModuleConfig.chartOfAccountFeeIncomeParentCode,
      })
    }
  }, [creditModuleConfig])

  return (
    <Dialog open={open} onOpenChange={() => setOpen(false)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("credit.setTitle")}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col space-y-2">
          {Object.entries(formData).map(([key, value]) => (
            <div key={key}>
              <Label htmlFor={key}>{t(`credit.${key}`)}</Label>
              <Input
                id={key}
                value={value}
                onChange={(e) => setFormData({ ...formData, [key]: e.target.value })}
              />
            </div>
          ))}
        </div>
        {updateCreditConfigError && (
          <div className="text-destructive">{updateCreditConfigError.message}</div>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={() => setOpen(false)}>
            {tCommon("cancel")}
          </Button>
          <Button
            loading={updateCreditConfigLoading}
            onClick={async () => {
              await updateCreditConfig({ variables: { input: formData } })
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
