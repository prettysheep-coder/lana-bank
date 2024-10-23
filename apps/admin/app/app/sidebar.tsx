"use client"

import Icon from "@/components/icon"

const Sidebar = () => {
  return (
    <div className="md:block md:w-auto px-4 py-2 border-b md:border-0 w-full sticky top-0 bg-white">
      <div className="flex items-center space-x-2">
        <Icon className="w-5" />
      </div>
    </div>
  )
}

export default Sidebar
