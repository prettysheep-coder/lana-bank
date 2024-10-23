import Caraousel from "./caraousel"

import Card from "@/components/card"

const AuthLayout: React.FC<React.PropsWithChildren> = async ({ children }) => (
  <div className="flex flex-auto h-screen">
    <div className="hidden sm:block w-2/5 p-2">
      <Card className="bg-primary w-full h-full rounded-md">
        <Caraousel />
      </Card>
    </div>
    <div className="sm:w-3/5 sm:p-10">
      <div className="sm:hidden absolute w-full rounded-md p-2">
        <div className="bg-primary w-full h-4 rounded-sm"></div>
      </div>
      {children}
    </div>
  </div>
)

export default AuthLayout
