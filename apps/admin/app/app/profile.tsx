"use client"

import React, { useEffect, useRef, useState } from "react"
import { gql } from "@apollo/client"
import { LuChevronsUpDown } from "react-icons/lu"
import { signOut } from "next-auth/react"

import { useMeQuery } from "@/lib/graphql/generated"
import Button from "@/components/button"

gql`
  query Me {
    me {
      userId
      email
      roles
    }
  }
`

const Profile = () => {
  const { data, loading } = useMeQuery()
  const [isDropdownOpen, setDropdownOpen] = useState(false)
  const dropdownRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false)
      }
    }

    document.addEventListener("mousedown", handleClickOutside)
    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [dropdownRef])

  if (!data || loading) return <></>
  const avatar = data.me?.email.replace(/@/, "").substring(0, 2).toUpperCase()

  return (
    <div ref={dropdownRef} className="relative">
      <div
        className="flex items-center justify-center space-x-2 border-2 p-1 rounded-md cursor-pointer hover:bg-grey-5"
        onClick={() => setDropdownOpen(!isDropdownOpen)}
      >
        <LuChevronsUpDown className="text-secondary text-xl" />
        <div className="bg-secondary text-grey-5 text-md font-medium p-1 rounded-full">
          {avatar}
        </div>
      </div>
      {isDropdownOpen && (
        <div className="absolute right-0 mt-2 w-60 bg-white border border-grey-5 rounded-md shadow-lg z-50">
          <div className="px-4 pt-2">
            <p>
              <strong>User ID:</strong> {data.me.userId}
            </p>
            <p>
              <strong>Email:</strong> {data.me.email}
            </p>
            <p>
              <strong>Roles:</strong> {data.me.roles.join(", ")}
            </p>
          </div>
          <div className="w-full p-2 flex justify-end">
            <Button onClick={signOut} title="Logout" />
          </div>
        </div>
      )}
    </div>
  )
}

export default Profile
