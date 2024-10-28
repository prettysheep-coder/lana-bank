"use client"

import React, { useState } from "react"
import Link from "next/link"

import { RemoveUserCommitteeDialog } from "../remove-user"

import { GetCommitteeDetailsQuery, useMeQuery } from "@/lib/graphql/generated"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/primitive/card"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/primitive/table"
import { Button } from "@/components/primitive/button"

type CommitteeUsersProps = {
  committee: NonNullable<GetCommitteeDetailsQuery["committee"]>
}

type UserToRemove = {
  userId: string
  email: string
} | null

export const CommitteeUsers: React.FC<CommitteeUsersProps> = ({ committee }) => {
  const [userToRemove, setUserToRemove] = useState<UserToRemove>(null)

  return (
    <>
      {committee.users.length > 0 ? (
        <Card>
          <CardHeader>
            <CardTitle>Committee Members</CardTitle>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Email</TableHead>
                  <TableHead>User ID</TableHead>
                  <TableHead></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {committee.users.map((user) => (
                  <TableRow key={user.userId}>
                    <TableCell>
                      <Link href={`/users/${user.userId}`}>{user.email}</Link>
                    </TableCell>
                    <TableCell>{user.userId}</TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        onClick={() =>
                          setUserToRemove({
                            userId: user.userId,
                            email: user.email,
                          })
                        }
                      >
                        Remove member
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardContent>
            <p className="mt-6">No members found</p>
          </CardContent>
        </Card>
      )}
      {userToRemove && (
        <RemoveUserCommitteeDialog
          committeeId={committee.committeeId}
          userId={userToRemove.userId}
          userEmail={userToRemove.email}
          openRemoveUserDialog={Boolean(userToRemove)}
          setOpenRemoveUserDialog={() => setUserToRemove(null)}
        />
      )}
    </>
  )
}
