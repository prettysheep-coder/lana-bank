"use client"

import React, { useState } from "react"
import { toast } from "sonner"
import { gql } from "@apollo/client"
import { useTranslations } from "next-intl"

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@lana/web/ui/dialog"

import { Input } from "@lana/web/ui/input"
import { Button } from "@lana/web/ui/button"
import { Label } from "@lana/web/ui/label"

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@lana/web/ui/select"

import { Plus } from "lucide-react"

import {
  DebitOrCredit,
  ManualTransactionEntryInput,
  ManualTransactionExecuteInput,
  useExecuteManualTransactionMutation,
} from "@/lib/graphql/generated"
import { useModalNavigation } from "@/hooks/use-modal-navigation"
import DataTable from "@/components/data-table"

gql`
  mutation ExecuteManualTransaction($input: ManualTransactionExecuteInput!) {
    manualTransactionExecute(input: $input) {
      transaction {
        id
        ledgerTransactionId
        createdAt
        description
      }
    }
  }
`

type ExecuteManualTransactionProps = {
  setOpenExecuteManualTransaction: (isOpen: boolean) => void
  openExecuteManualTransaction: boolean
}

export const ExecuteManualTransactionDialog: React.FC<ExecuteManualTransactionProps> = ({
  setOpenExecuteManualTransaction,
  openExecuteManualTransaction,
}) => {
  const t = useTranslations("ManualTransactions")
  const { navigate, isNavigating } = useModalNavigation({
    closeModal: () => {
      setOpenExecuteManualTransaction(false)
      resetForm()
    },
  })

  const [
    executeManualTransaction,
    { loading, reset, error: executeManualTransactionError },
  ] = useExecuteManualTransactionMutation({})

  const isLoading = loading || isNavigating

  const [
    openExecuteManualTransactionEntryInput,
    setOpenExecuteManualTransactionEntryInput,
  ] = useState(false)
  const [formValues, setFormValues] = useState<ManualTransactionExecuteInput>({
    description: "",
    reference: "",
    entries: [],
  })
  const addEntry = (entry: ManualTransactionEntryInput) => {
    setFormValues((prevValues) => ({
      ...prevValues,
      entries: [...prevValues.entries, entry],
    }))
  }

  const [error, setError] = useState<string | null>(null)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormValues((prevValues) => ({
      ...prevValues,
      [name]: value,
    }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    try {
      await executeManualTransaction({
        variables: {
          input: { ...formValues },
        },
        onCompleted: (data) => {
          if (data?.manualTransactionExecute.transaction) {
            toast.success(t("success"))
            navigate(
              `/ledger-transaction/${data.manualTransactionExecute.transaction.ledgerTransactionId}`,
            )
          } else {
            throw new Error(t("errored"))
          }
        },
      })
    } catch (error) {
      console.error("Error executing manual transaction:", error)
      if (error instanceof Error) {
        setError(error.message)
      } else if (executeManualTransactionError) {
        setError(executeManualTransactionError.message)
      } else {
        setError(t("errored"))
      }
      toast.error(t("errored"))
    }
  }

  const resetForm = () => {
    setFormValues({
      description: "",
      reference: "",
      entries: [],
    })
    setError(null)
    reset()
  }

  return (
    <>
      <Dialog
        open={openExecuteManualTransaction}
        onOpenChange={(isOpen) => {
          setOpenExecuteManualTransaction(isOpen)
          if (!isOpen) {
            resetForm()
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("title")}</DialogTitle>
            <DialogDescription>{t("description")}</DialogDescription>
          </DialogHeader>
          <form className="flex flex-col gap-4" onSubmit={handleSubmit}>
            <div>
              <Label htmlFor="description">{t("fields.description")}</Label>
              <Input
                id="description"
                name="description"
                type="text"
                required
                placeholder={t("placeholders.description")}
                value={formValues.description}
                onChange={handleChange}
                disabled={isLoading}
                data-testid="execute-manual-transaction-description-input"
              />
            </div>
            <div>
              <Label htmlFor="reference">{t("fields.reference")}</Label>
              <Input
                id="reference"
                name="reference"
                type="text"
                required
                placeholder={t("placeholders.reference")}
                value={formValues.reference || ""}
                onChange={handleChange}
                disabled={isLoading}
                data-testid="execute-manual-transaction-description-input"
              />
            </div>

            <div>
              <div className="flex justify-between items-center w-full">
                <Label htmlFor="entries">{t("fields.entries")}</Label>
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => {
                    setOpenExecuteManualTransactionEntryInput(true)
                  }}
                  disabled={isLoading}
                  data-testid="execute-manual-transaction-entry-input-button"
                >
                  <Plus />
                  {t("addEntryBtn")}
                </Button>
              </div>
              <DataTable
                autoFocus={false}
                data={formValues.entries}
                emptyMessage={t("noEntries")}
                columns={[
                  {
                    key: "accountRef",
                    header: t("table.accountRef"),
                  },
                  {
                    key: "amount",
                    header: t("table.amount"),
                  },
                  {
                    key: "currency",
                    header: t("table.currency"),
                  },
                  {
                    key: "direction",
                    header: t("table.direction"),
                  },
                  {
                    key: "description",
                    header: t("table.description"),
                  },
                ]}
              />
            </div>

            {error && <p className="text-destructive">{error}</p>}

            <DialogFooter>
              <Button
                type="submit"
                loading={isLoading}
                data-testid="execute-manual-transaction-submit-button"
              >
                {t("execute")}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
      <ManualTransactionsEntryInput
        setOpenExecuteManualTransactionEntryInput={
          setOpenExecuteManualTransactionEntryInput
        }
        openExecuteManualTransactionEntryInput={openExecuteManualTransactionEntryInput}
        addEntry={addEntry}
      />
    </>
  )
}

type ManualTransactionsEntryInputProps = {
  setOpenExecuteManualTransactionEntryInput: (isOpen: boolean) => void
  openExecuteManualTransactionEntryInput: boolean
  addEntry: (entry: ManualTransactionEntryInput) => void
}

const ManualTransactionsEntryInput: React.FC<ManualTransactionsEntryInputProps> = ({
  setOpenExecuteManualTransactionEntryInput,
  openExecuteManualTransactionEntryInput,
  addEntry,
}) => {
  const t = useTranslations("ManualTransactions.EntryInput")
  const { isNavigating } = useModalNavigation({
    closeModal: () => {
      setOpenExecuteManualTransactionEntryInput(false)
      resetForm()
    },
  })

  const [formValues, setFormValues] = useState<ManualTransactionEntryInput>({
    accountRef: "",
    amount: 0.0,
    currency: "USD",
    direction: DebitOrCredit.Credit,
    description: "",
  })

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormValues((prevValues) => ({
      ...prevValues,
      [name]: value,
    }))
  }

  const resetForm = () => {
    setFormValues({
      accountRef: "",
      amount: 0.0,
      currency: "USD",
      direction: DebitOrCredit.Credit,
      description: "",
    })
  }

  return (
    <Dialog
      open={openExecuteManualTransactionEntryInput}
      onOpenChange={setOpenExecuteManualTransactionEntryInput}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("title")}</DialogTitle>
          <DialogDescription>{t("description")}</DialogDescription>
        </DialogHeader>
        <form
          className="flex flex-col gap-4"
          onSubmit={(e) => {
            e.preventDefault()
            addEntry(formValues)
            setOpenExecuteManualTransactionEntryInput(false)
          }}
        >
          <div>
            <Label htmlFor="accountRef">{t("fields.accountRef")}</Label>
            <Input
              id="accountRef"
              name="accountRef"
              type="text"
              required
              placeholder={t("placeholders.accountRef")}
              value={formValues.accountRef}
              onChange={handleChange}
            />
          </div>
          <div>
            <Label htmlFor="amount">{t("fields.amount")}</Label>
            <Input
              id="amount"
              name="amount"
              type="number"
              required
              placeholder={t("placeholders.amount")}
              value={formValues.amount}
              onChange={handleChange}
            />
          </div>
          <div>
            <Label htmlFor="currency">{t("fields.currency")}</Label>
            <Select
              value={formValues.currency}
              onValueChange={(value) => {
                setFormValues((prevValues) => ({
                  ...prevValues,
                  currency: value,
                }))
              }}
            >
              <SelectTrigger id="currency" data-testid="currency">
                <SelectValue placeholder={t("placeholders.currency")} />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={"USD"}>{t("fields.usd")}</SelectItem>
                <SelectItem value={"BTC"}>{t("fields.btc")}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div>
            <Label htmlFor="direction">{t("fields.direction")}</Label>
            <Select
              value={formValues.direction}
              onValueChange={(value) => {
                setFormValues((prevValues) => ({
                  ...prevValues,
                  direction: value as DebitOrCredit,
                }))
              }}
            >
              <SelectTrigger id="direction" data-testid="direction">
                <SelectValue placeholder={t("placeholders.direction")} />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={DebitOrCredit.Debit}>{t("fields.debit")}</SelectItem>
                <SelectItem value={DebitOrCredit.Credit}>{t("fields.credit")}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div>
            <Label htmlFor="description">{t("fields.description")}</Label>
            <Input
              id="description"
              name="description"
              type="text"
              required
              placeholder={t("placeholders.description")}
              value={formValues.description}
              onChange={handleChange}
            />
          </div>
          <DialogFooter>
            <Button
              type="submit"
              loading={isNavigating}
              data-testid="execute-manual-transaction-entry-input-submit-button"
            >
              {t("add")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
