import Image from "next/image"

import { BASE_PATH } from "@/app/config"

type IconProps = {
  className?: string
}

const Icon: React.FC<IconProps> = ({ className }) => (
  <Image
    className={className}
    src={`${BASE_PATH}/icon.svg`}
    alt="Lava Bank Icon"
    width="47"
    height="75"
  />
)

export default Icon
