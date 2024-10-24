"use client"

import Image from "next/image"
import { Carousel as MTCarousel } from "@material-tailwind/react"

import { BASE_PATH } from "@/app/config"

const Carousel: React.FC = () => {
  return (
    <div className="px-10 flex h-full">
      <MTCarousel
        placeholder={undefined}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        loop
        autoplay
        prevArrow={() => <></>}
        nextArrow={() => <></>}
        navigation={({ setActiveIndex, activeIndex, length }) => (
          <div className="absolute bottom-20 left-2/4 z-50 flex -translate-x-2/4 gap-2">
            {new Array(length).fill("").map((_, i) => (
              <span
                key={i}
                className={`block h-1 cursor-pointer rounded-2xl transition-all content-[''] ${
                  activeIndex === i ? "w-8 bg-white" : "w-4 bg-white/50"
                }`}
                onClick={() => setActiveIndex(i)}
              />
            ))}
          </div>
        )}
      >
        <CarouselItem
          text="One-stop view into the bank’s financials"
          icon="auth-carousel/onestop.svg"
        />
        <CarouselItem
          text="Manage customers, approve loans, record deposits and withdrawals"
          icon="auth-carousel/manage.svg"
        />
        <CarouselItem
          text="Generate regulatory reporting for government compliance"
          icon="auth-carousel/generate.svg"
        />
      </MTCarousel>
    </div>
  )
}

export default Carousel

type CaraouselItemProps = {
  text: string
  icon: string
}
const CarouselItem: React.FC<CaraouselItemProps> = ({ text, icon }) => (
  <div className="flex flex-col justify-center items-center space-y-10 h-full">
    <Image src={`${BASE_PATH}/${icon}`} alt={text} width="300" height="300" />
    <div className="text-title text-grey-5">{text}</div>
  </div>
)
