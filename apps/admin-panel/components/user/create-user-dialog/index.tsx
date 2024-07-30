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
import { useUserCreateMutation } from "@/lib/graphql/generated"
import { Input } from "@/components/primitive/input"
import { Button } from "@/components/primitive/button"
import { Label } from "@/components/primitive/label"

gql`
  mutation UserCreate($input: UserCreateInput!) {
    userCreate(input: $input) {
      user {
        userId
        email
        roles
      }
    }
  }
`

function CreateUserDialog({
  setOpenCreateUserDialog,
  openCreateUserDialog,
  refetch,
}: {
  setOpenCreateUserDialog: (isOpen: boolean) => void
  openCreateUserDialog: boolean
  refetch: () => void
}) {
  const [createUser, { loading, reset }] = useUserCreateMutation()
  const [email, setEmail] = useState<string>("")

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await createUser({
        variables: {
          input: {
            email,
          },
        },
      })
      refetch()
      toast.success("User created successfully")
      setOpenCreateUserDialog(false)
    } catch (error) {
      console.log(error)
    }
  }

  return (
    <Dialog
      open={openCreateUserDialog}
      onOpenChange={(isOpen) => {
        setOpenCreateUserDialog(isOpen)
        if (!isOpen) {
          reset()
        }
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add new User</DialogTitle>
          <DialogDescription>
            Add a new user to the admin-panel by providing their email address
          </DialogDescription>
        </DialogHeader>
        <form className="flex flex-col gap-4" onSubmit={handleSubmit}>
          <div>
            <Label>Email</Label>
            <Input
              type="email"
              required
              placeholder="Please enter the email address"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </div>
          <DialogFooter>
            <Button loading={loading}>Submit</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}

export default CreateUserDialog
