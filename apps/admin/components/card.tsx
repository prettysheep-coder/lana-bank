type CardProps = {
  className?: string
}

const Card: React.FC<React.PropsWithChildren<CardProps>> = ({ className, children }) => {
  return <div className={className}>{children}</div>
}

export default Card
