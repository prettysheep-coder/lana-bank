"use client"

import { gql } from "@apollo/client"

import React from "react"

import { CreditFacilityCollateralUpdateDialog } from "../collateral-update"

import { CreditFacilityApproveDialog } from "../approve"

import { CreditFacilityDisbursementInitiateDialog } from "../disbursement-Initiate"

import { useGetCreditFacilityDetailsQuery } from "@/lib/graphql/generated"
import { DetailItem, DetailsGroup } from "@/components/details"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/primitive/card"
import Balance from "@/components/balance/balance"
import { formatCollateralizationState } from "@/lib/utils"

import { Button } from "@/components/primitive/button"

gql`
  query GetCreditFacilityDetails($id: UUID!) {
    creditFacility(id: $id) {
      id
      creditFacilityId
      collateralizationState
      balance {
        outstanding {
          usdBalance
        }
      }
    }
  }
`

type CreditFacilityDetailsProps = { creditFacilityId: string }

const CreditFacilityDetailsCard: React.FC<CreditFacilityDetailsProps> = ({
  creditFacilityId,
}) => {
  const [openCollateralUpdateDialog, setOpenCollateralUpdateDialog] =
    React.useState(false)
  const [openDisbursementInitiateDialog, setOpenDisbursementInitiateDialog] =
    React.useState(false)
  const [openApproveDialog, setOpenApproveDialog] = React.useState(false)

  const {
    data: creditFacilityDetails,
    loading,
    error,
  } = useGetCreditFacilityDetailsQuery({
    variables: { id: creditFacilityId },
  })

  return (
    <div className="flex gap-4">
      <Card className="w-11/12">
        {loading ? (
          <CardContent className="pt-6">Loading...</CardContent>
        ) : error ? (
          <CardContent className="pt-6 text-destructive">{error.message}</CardContent>
        ) : creditFacilityDetails?.creditFacility ? (
          <>
            <CardHeader>
              <CardTitle>Credit Facility Overview</CardTitle>
            </CardHeader>
            <CardContent>
              <DetailsGroup>
                <DetailItem
                  label="Credit Facility ID"
                  value={creditFacilityDetails.creditFacility.creditFacilityId}
                />
                <DetailItem
                  label="Outstanding Balance"
                  valueComponent={
                    <Balance
                      amount={
                        creditFacilityDetails.creditFacility.balance.outstanding
                          .usdBalance
                      }
                      currency="usd"
                    />
                  }
                />
                <DetailItem
                  label="Collateralization State"
                  value={formatCollateralizationState(
                    creditFacilityDetails.creditFacility.collateralizationState,
                  )}
                />
              </DetailsGroup>
            </CardContent>
          </>
        ) : (
          creditFacilityId &&
          !creditFacilityDetails?.creditFacility?.creditFacilityId && (
            <CardContent className="pt-6">
              No credit facility found with this ID
            </CardContent>
          )
        )}
      </Card>
      <div className="flex flex-col space-y-2 mt-1">
        <Button
          variant="primary"
          className="w-full"
          onClick={() => setOpenApproveDialog(true)}
        >
          Approve
        </Button>
        <Button
          variant="primary"
          className="w-full"
          onClick={() => setOpenCollateralUpdateDialog(true)}
        >
          Collateral Update
        </Button>
        <Button
          variant="primary"
          className="w-full"
          onClick={() => setOpenDisbursementInitiateDialog(true)}
        >
          Disbursement Initiate
        </Button>
      </div>

      <CreditFacilityCollateralUpdateDialog
        creditFacilityId={creditFacilityId}
        openDialog={openCollateralUpdateDialog}
        setOpenDialog={() => setOpenCollateralUpdateDialog(false)}
      />
      <CreditFacilityDisbursementInitiateDialog
        creditFacilityId={creditFacilityId}
        openDialog={openDisbursementInitiateDialog}
        setOpenDialog={() => setOpenDisbursementInitiateDialog(false)}
      />
      <CreditFacilityApproveDialog
        creditFacilityId={creditFacilityId}
        openDialog={openApproveDialog}
        setOpenDialog={() => setOpenApproveDialog(false)}
      />
    </div>
  )
}

export default CreditFacilityDetailsCard
