import { useRouter, usePathname } from "next/navigation"

type Tab = {
  url: string
  tabLabel: string
}

export function useTabNavigation(tabs: Tab[], entityPath: string, entityId: string) {
  const router = useRouter()
  const pathname = usePathname()

  const getCurrentTab = () => {
    if (pathname === `${entityPath}/${entityId}`) return tabs[0].url
    const path = pathname.split(`${entityPath}/${entityId}`)[1]
    return path || tabs[0].url
  }

  const handleTabChange = (value: string) => {
    const path =
      value === tabs[0].url
        ? `${entityPath}/${entityId}`
        : `${entityPath}/${entityId}${value}`
    router.push(path)
  }

  return {
    currentTab: getCurrentTab(),
    handleTabChange,
  }
}
