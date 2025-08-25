variable "region" {
  type    = string
  default = "ap-southeast-2"
}

variable "service_name" {
  type    = string
  default = "tui-li"
}

variable "power" {
  type    = string
  default = "nano"   # nano|micro|small|medium|large|xlarge
}

variable "scale" {
  type    = number
  default = 1
}

variable "container_name" {
  type    = string
  default = "api"
}

variable "container_port" {
  type    = number
  default = 3000
}

variable "health_check_path" {
  type    = string
  default = "/health"
}

variable "container_image" {
  type = string
  description = "Public Docker image reference (Docker Hub, ECR, etc.)"
}
