import React, { useState } from "react"
import { gql } from "@apollo/client"
import { toast } from "sonner"

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/primitive/dialog"
import {
  CommitteesDocument,
  GetCommitteeDetailsDocument,
  useCommitteeAddUserMutation,
} from "@/lib/graphql/generated"
import { Button } from "@/components/primitive/button"
import { Label } from "@/components/primitive/label"
import { Input } from "@/components/primitive/input"

gql`
  mutation CommitteeAddUser($input: CommitteeAddUserInput!) {
    committeeAddUser(input: $input) {
      committee {
        id
        committeeId
        users {
          userId
          email
          roles
        }
      }
    }
  }
`

type AddUserCommitteeDialogProps = {
  committeeId: string
  setOpenAddUserDialog: (isOpen: boolean) => void
  openAddUserDialog: boolean
  refetch?: () => void
}

export const AddUserCommitteeDialog: React.FC<AddUserCommitteeDialogProps> = ({
  committeeId,
  setOpenAddUserDialog,
  openAddUserDialog,
  refetch,
}) => {
  const [addUser, { loading, reset, error: addUserError }] = useCommitteeAddUserMutation()

  const [userId, setUserId] = useState("")
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    try {
      const { data } = await addUser({
        variables: {
          input: {
            committeeId,
            userId,
          },
        },
        refetchQueries: [CommitteesDocument, GetCommitteeDetailsDocument],
      })

      if (data?.committeeAddUser.committee) {
        toast.success("User added to committee successfully")
        if (refetch) refetch()
        setOpenAddUserDialog(false)
      } else {
        throw new Error("Failed to add user to committee. Please try again.")
      }
    } catch (error) {
      console.error("Error adding user to committee:", error)
      if (error instanceof Error) {
        setError(error.message)
      } else if (addUserError?.message) {
        setError(addUserError.message)
      } else {
        setError("An unexpected error occurred. Please try again.")
      }
      toast.error("Failed to add user to committee")
    }
  }

  const resetForm = () => {
    setUserId("")
    setError(null)
    reset()
  }

  return (
    <Dialog
      open={openAddUserDialog}
      onOpenChange={(isOpen) => {
        setOpenAddUserDialog(isOpen)
        if (!isOpen) {
          resetForm()
        }
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add User to Committee</DialogTitle>
          <DialogDescription>
            Enter the user ID to add to this committee
          </DialogDescription>
        </DialogHeader>
        <form className="flex flex-col gap-4" onSubmit={handleSubmit}>
          <div>
            <Label htmlFor="userId">User ID</Label>
            <Input
              id="userId"
              name="userId"
              type="text"
              required
              placeholder="Enter user ID"
              value={userId}
              onChange={(e) => setUserId(e.target.value)}
            />
          </div>

          {error && <p className="text-destructive">{error}</p>}

          <DialogFooter>
            <Button type="submit" loading={loading}>
              Add User
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
