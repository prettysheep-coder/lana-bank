"use client"

import { toast } from "sonner"

import { Button } from "@/components"

const Page = () => {
  return (
    <>
      <div>PLS WORK</div>
      <Button title="Click me" onClick={() => toast.success("WORKS")} />
    </>
  )
}

export default Page
