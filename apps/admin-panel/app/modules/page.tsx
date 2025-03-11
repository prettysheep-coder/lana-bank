"use client"

import { useState } from "react"
import { useTranslations } from "next-intl"
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@lana/web/ui/card"
import { gql } from "@apollo/client"

import { Button } from "@lana/web/ui/button"
import { Separator } from "@lana/web/ui/separator"
import { LoaderCircle, Pencil } from "lucide-react"

import { DepositConfigUpdateDialog } from "./deposit-config-update"

import { DetailItem } from "@/components/details"
import { useDepositConfigQuery } from "@/lib/graphql/generated"

gql`
  query depositConfig {
    depositConfig {
      chartOfAccountsId
      chartOfAccountsDepositAccountsParentCode
      chartOfAccountsOmnibusParentCode
    }
  }
`

const Modules: React.FC = () => {
  const t = useTranslations("Modules")
  const tCommon = useTranslations("Common")

  const [openDepositConfigUpdateDialog, setOpenDepositConfigUpdateDialog] =
    useState(false)

  const { data: depositConfig, loading: depositConfigLoading } = useDepositConfigQuery()

  return (
    <>
      <DepositConfigUpdateDialog
        open={openDepositConfigUpdateDialog}
        setOpen={setOpenDepositConfigUpdateDialog}
        depositModuleConfig={depositConfig?.depositConfig || undefined}
      />
      <Card>
        <CardHeader>
          <CardTitle>{t("deposit.title")}</CardTitle>
          <CardDescription>{t("deposit.description")}</CardDescription>
        </CardHeader>

        <CardContent>
          {depositConfigLoading ? (
            <LoaderCircle className="animate-spin" />
          ) : depositConfig ? (
            <>
              <DetailItem
                label="Chart of Accounts ID"
                value={depositConfig?.depositConfig?.chartOfAccountsId}
              />
              <DetailItem
                label="Chart of Accounts Deposit Accounts Parent Code"
                value={
                  depositConfig?.depositConfig?.chartOfAccountsDepositAccountsParentCode
                }
              />
              <DetailItem
                label="Chart of Accounts Omnibus Parent Code"
                value={depositConfig?.depositConfig?.chartOfAccountsOmnibusParentCode}
              />
            </>
          ) : (
            <div>{t("notYetAssigned")}</div>
          )}
        </CardContent>
        <Separator className="mb-4" />
        <CardFooter className="-mb-3 -mt-1 justify-end">
          <Button
            variant="outline"
            onClick={() => setOpenDepositConfigUpdateDialog(true)}
          >
            <Pencil />
            {depositConfig ? tCommon("edit") : tCommon("set")}
          </Button>
        </CardFooter>
      </Card>
    </>
  )
}

export default Modules
