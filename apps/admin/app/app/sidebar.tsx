"use client"

import { useState } from "react"
import { RiMenu2Fill } from "react-icons/ri"

import { Drawer } from "@material-tailwind/react"

import Profile from "./profile"

import Icon from "@/components/icon"

const Sidebar = () => {
  const [open, setOpen] = useState(false)
  return (
    <>
      <div className="sticky w-full md:block md:min-w-64 md:w-auto px-4 py-2 border-b border-grey-4 md:border-0 md:h-screen md:self-start">
        <div className="flex items-center space-x-2 justify-between">
          <div
            className="hover:bg-grey-5 md:hidden p-1 rounded-md"
            onClick={() => setOpen((o) => !o)}
          >
            <RiMenu2Fill className="text-secondary text-2xl" />
          </div>
          <Icon className="w-5" />
          <Profile />
        </div>
        <div className="hidden md:block">
          <Menus />
        </div>
      </div>
      <Drawer
        open={open}
        onClose={() => setOpen(false)}
        placeholder={undefined}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
      >
        <Menus />
      </Drawer>
    </>
  )
}

export default Sidebar

const Menus = () => (
  <nav>
    <ul>A</ul>
    <ul>A</ul>
    <ul>A</ul>
    <ul>A</ul>
    <ul>A</ul>
  </nav>
)
