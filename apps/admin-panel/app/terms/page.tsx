import React from "react"

import { PageHeading } from "@/components/page-heading"
import { Button } from "@/components/primitive/button"
import { UpdateCurrentTermDialog } from "@/components/terms/update-current-terms-dialog"

function TermPage() {
  return (
    <main>
      <PageHeading>Terms</PageHeading>
      <UpdateCurrentTermDialog>
        <Button>Update Current Term</Button>
      </UpdateCurrentTermDialog>
    </main>
  )
}

export default TermPage
