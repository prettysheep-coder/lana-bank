"use client"
import React, { useState } from "react"
import Link from "next/link"

import { CrossIcon, LavaBankIcon, PersonIcon } from "../icons"
import { Card, CardContent, CardHeader, CardTitle } from "../primitive/card"

export default function NavBar() {
  const [openMenu, setOpenMenu] = useState(false)

  return (
    <>
      <nav
        className={`max-w-[70rem] m-auto flex justify-between items-center mt-2 relative `}
      >
        <div className="flex items-center gap-4">
          <Link href="/">
            <LavaBankIcon />
          </Link>
          <p className="mt-4">Bitcoin Backed Loans</p>
        </div>
        <div className="flex items-center gap-4 p-4">
          <p>Sdiddahrt@g.com</p>
          <div className="border border-primary p-2 rounded-full cursor-pointer">
            <PersonIcon
              onClick={() => {
                setOpenMenu(true)
              }}
              className="w-6 h-6"
            />
            {openMenu && (
              <div className="absolute right-0 top-0 z-20">
                <Card className="w-80 border-none">
                  <div className=" flex justify-between items-center p-4">
                    <div
                      onClick={() => {
                        setOpenMenu(false)
                      }}
                    >
                      <CrossIcon className="w-6 h-6 " />
                    </div>
                    <div className="flex justify-end items-center gap-4">
                      <p>sid@g.com</p>
                      <div className="border border-primary p-2 rounded-full">
                        <PersonIcon className="w-6 h-6" />
                      </div>
                    </div>
                  </div>
                  <Card variant="transparent">
                    <CardHeader className="pt-0 pb-4">
                      <CardTitle>Account</CardTitle>
                    </CardHeader>
                    <CardContent className="p-6 pt-0 flex flex-col gap-2 text-sm">
                      <div className="flex  justify-between">
                        <p className="text-textColor-secondary">Email</p>
                        <p>sid@g.com</p>
                      </div>
                      <div className="flex  justify-between">
                        <p className="text-textColor-secondary">Two-Factor Auth</p>
                        <p>Enabled</p>
                      </div>
                    </CardContent>
                  </Card>
                </Card>
              </div>
            )}
          </div>
        </div>
      </nav>
      {openMenu && (
        <div
          onClick={() => setOpenMenu(false)}
          className="fixed inset-0 bg-black bg-opacity-65 z-10"
        />
      )}
    </>
  )
}
