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
import { FormEvent, useEffect, useState } from "react"

import {
  CreditConfigDocument,
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

const initialFormData = {
  chartOfAccountFacilityOmnibusParentCode: "",
  chartOfAccountCollateralOmnibusParentCode: "",
  chartOfAccountFacilityParentCode: "",
  chartOfAccountCollateralParentCode: "",
  chartOfAccountDisbursedReceivableParentCode: "",
  chartOfAccountInterestReceivableParentCode: "",
  chartOfAccountInterestIncomeParentCode: "",
  chartOfAccountFeeIncomeParentCode: "",
}

export const CreditConfigUpdateDialog: React.FC<CreditConfigUpdateDialogProps> = ({
  open,
  setOpen,
  creditModuleConfig,
}) => {
  const t = useTranslations("Modules")
  const tCommon = useTranslations("Common")

  const [updateCreditConfig, { loading, error, reset }] = useCreditConfigUpdateMutation({
    refetchQueries: [CreditConfigDocument]
  })
  const [formData, setFormData] = useState<CreditModuleConfigUpdateInput>(initialFormData)

  const close = () => {
    reset()
    setOpen(false)
    setFormData(initialFormData)
  }

  useEffect(() => {
    if (
      creditModuleConfig &&
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

  const submit = async (e: FormEvent) => {
    e.preventDefault()
    await updateCreditConfig({ variables: { input: formData } })
    setOpen(false)
  }

  return (
    <Dialog open={open} onOpenChange={close}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("credit.setTitle")}</DialogTitle>
        </DialogHeader>
        <form onSubmit={submit}>
          <div className="flex flex-col space-y-2 w-full">
            {Object.entries(formData).map(([key, value]) => (
              <div key={key}>
                <Label htmlFor={key}>{t(`credit.${key}`)}</Label>
                <Input
                  id={key}
                  value={value.replace(/\./g, "")}
                  onChange={(e) => setFormData({ ...formData, [key]: e.target.value })}
                  required={true}
                />
              </div>
            ))}
          </div>
          {error && <div className="text-destructive">{error.message}</div>}
          <DialogFooter className="mt-4">
            <Button variant="outline" onClick={close}>
              {tCommon("cancel")}
            </Button>
            <Button loading={loading} type="submit">
              {tCommon("save")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
