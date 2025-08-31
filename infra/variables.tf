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
  default = "tui-li-api"
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
  default     = "glexposito/tui-li-api:latest"
}

variable "app_aws_access_key_id" {
  type      = string
  sensitive = true
}

variable "app_aws_secret_access_key" {
  type      = string
  sensitive = true
}

# ===== DynamoDB =====
variable "dynamodb_table_name" {
  type    = string
  default = "tui-li-urls"
}

# (Optional) if you want TTL GC
variable "dynamodb_ttl_enabled" {
  type    = bool
  default = true
}

variable "dynamodb_ttl_attr" {
  type    = string
  default = "ttl"
}
