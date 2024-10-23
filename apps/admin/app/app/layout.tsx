import Sidebar from "./sidebar"

const AppLayout: React.FC<React.PropsWithChildren> = async ({ children }) => {
  return (
    <div className="h-screen w-screen md:overflow-hidden flex md:flex-col flex-wrap items-center justify-between mx-auto">
      <Sidebar />
      <div className="p-2 overflow-auto">{children}</div>
    </div>
  )
}

export default AppLayout
