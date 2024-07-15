"use client"
import React from "react"

import { gql } from "@apollo/client"

import { PiPencilSimpleLineLight } from "react-icons/pi"

import { PageHeading } from "@/components/page-heading"
import { Button } from "@/components/primitive/button"
import { UpdateCurrentTermDialog } from "@/components/terms/update-current-terms-dialog"
import { useCurrentTermsQuery } from "@/lib/graphql/generated"
import { Card, CardContent, CardHeader } from "@/components/primitive/card"
import { DetailItem, DetailsGroup } from "@/components/details"
import { formatInterval, formatPeriod } from "@/lib/term/utils"
import { Separator } from "@/components/primitive/separator"

gql`
  query CurrentTerms {
    currentTerms {
      id
      termsId
      values {
        annualRate
        interval
        liquidationCvl
        marginCallCvl
        initialCvl
        duration {
          period
          units
        }
      }
    }
  }
`

function TermPage() {
  const { data, loading, error, refetch } = useCurrentTermsQuery()

  return (
    <main className="max-w-[70rem] m-auto">
      <PageHeading>Terms</PageHeading>
      <Card>
        {loading ? (
          <CardContent className="pt-6">Loading...</CardContent>
        ) : error ? (
          <CardContent className="pt-6">Loading...</CardContent>
        ) : data && data.currentTerms ? (
          <>
            <CardHeader className="flex flex-row justify-between items-center mb-0">
              <div className="flex flex-col space-y-1.5">
                <h2 className="font-semibold leading-none tracking-tight">
                  Terms Detail
                </h2>
                <p className="text-textColor-secondary text-sm">
                  {data?.currentTerms.termsId}
                </p>
              </div>
              <UpdateCurrentTermDialog refetch={refetch}>
                <Button variant="secondary" className="mt-6 flex gap-2 items-center">
                  <PiPencilSimpleLineLight className="w-5 h-5" />
                  Update Current Term
                </Button>
              </UpdateCurrentTermDialog>
            </CardHeader>
            <Separator className="mb-4" />

            <CardContent>
              <DetailsGroup>
                <DetailItem
                  label="Duration"
                  value={
                    String(data.currentTerms.values.duration.units) +
                    " " +
                    formatPeriod(data.currentTerms.values.duration.period)
                  }
                />
                <DetailItem
                  label="Interval"
                  value={formatInterval(data.currentTerms.values.interval)}
                />
                <DetailItem
                  label="Annual Rate"
                  value={data.currentTerms.values.annualRate + "%"}
                />
                <DetailItem
                  label="Liquidation CVL"
                  value={data.currentTerms.values.liquidationCvl}
                />
                <DetailItem
                  label="Margin Call CVL"
                  value={data.currentTerms.values.marginCallCvl}
                />
                <DetailItem
                  label="Initial CVL"
                  value={data.currentTerms.values.initialCvl}
                />
              </DetailsGroup>
            </CardContent>
          </>
        ) : (
          <div className="flex justify-between items-center">
            <CardContent className="pt-6">No data found</CardContent>
            <UpdateCurrentTermDialog refetch={refetch}>
              <Button variant="secondary" className="flex gap-2 items-center mr-4">
                <PiPencilSimpleLineLight className="w-5 h-5" />
                Update Current Term
              </Button>
            </UpdateCurrentTermDialog>
          </div>
        )}
      </Card>
    </main>
  )
}

export default TermPage
