"use client"
import React, { useEffect, useState } from "react"
import { gql } from "@apollo/client"
import Link from "next/link"
import { useSearchParams } from "next/navigation"

import { CreateCommitteeDialog } from "./create"
import { AddUserCommitteeDialog } from "./add-user"

import { PageHeading } from "@/components/page-heading"
import { Button } from "@/components/primitive/button"
import { Committee, useMeQuery, useCommitteesQuery } from "@/lib/graphql/generated"
import { Card, CardContent } from "@/components/primitive/card"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/primitive/table"
import { formatDate } from "@/lib/utils"

gql`
  query Committees($first: Int!, $after: String) {
    committees(first: $first, after: $after) {
      pageInfo {
        hasPreviousPage
        hasNextPage
        startCursor
        endCursor
      }
      nodes {
        id
        committeeId
        createdAt
        users {
          userId
        }
      }
    }
  }
`

function CommitteesPage() {
  const searchParams = useSearchParams()

  const { data, refetch, loading, error, fetchMore } = useCommitteesQuery({
    variables: { first: 2 },
    fetchPolicy: "cache-and-network",
  })

  const [openCreateCommitteeDialog, setOpenCreateCommitteeDialog] =
    useState<boolean>(false)
  const [openAddUserDialog, setOpenAddUserDialog] = useState<Committee | null>(null)

  useEffect(() => {
    if (searchParams.get("create")) setOpenCreateCommitteeDialog(true)
  }, [searchParams, setOpenCreateCommitteeDialog])

  const { data: me } = useMeQuery()

  return (
    <main>
      {openAddUserDialog && (
        <AddUserCommitteeDialog
          committeeId={openAddUserDialog.committeeId}
          openAddUserDialog={Boolean(openAddUserDialog)}
          setOpenAddUserDialog={() => setOpenAddUserDialog(null)}
          refetch={refetch}
        />
      )}
      <CreateCommitteeDialog
        openCreateCommitteeDialog={openCreateCommitteeDialog}
        setOpenCreateCommitteeDialog={setOpenCreateCommitteeDialog}
        refetch={refetch}
      />

      <div className="flex justify-between items-center mb-8">
        <PageHeading className="mb-0">Committees</PageHeading>
        {me?.me.canCreateUser && (
          <Button onClick={() => setOpenCreateCommitteeDialog(true)}>Create New</Button>
        )}
      </div>

      <Card>
        <CardContent>
          {loading ? (
            <p className="mt-6">Loading...</p>
          ) : error ? (
            <p className="text-destructive mt-6">{error.message}</p>
          ) : data?.committees.nodes && data.committees.nodes.length > 0 ? (
            <Table className="mt-6">
              <TableHeader>
                <TableRow>
                  <TableHead>ID</TableHead>
                  <TableHead>Created</TableHead>
                  <TableHead>Members</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {data.committees.nodes.map((committee) => (
                  <TableRow key={committee.committeeId}>
                    <TableCell>
                      <Link href={`/committees/${committee.committeeId}`}>
                        {committee.committeeId}
                      </Link>
                    </TableCell>
                    <TableCell>{formatDate(committee.createdAt)}</TableCell>
                    <TableCell>{committee.users.length}</TableCell>
                  </TableRow>
                ))}
                {data.committees.pageInfo.hasNextPage && (
                  <TableRow
                    className="cursor-pointer"
                    onClick={() =>
                      fetchMore({
                        variables: {
                          after: data.committees.pageInfo.endCursor,
                        },
                      })
                    }
                  >
                    <TableCell>
                      <div className="font-thin italic">show more...</div>
                    </TableCell>
                    <TableCell />
                    <TableCell />
                  </TableRow>
                )}
              </TableBody>
            </Table>
          ) : (
            <p className="mt-6">No committees found</p>
          )}
        </CardContent>
      </Card>
    </main>
  )
}

export default CommitteesPage
