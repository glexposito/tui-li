# ===== Lightsail =====
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

variable "certificate_name" {
  description = "Existing Lightsail certificate name"
  type        = string
  default     = "tui-li-certificate"  # must match exactly in Lightsail
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
