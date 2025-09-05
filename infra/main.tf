# 1) Create the DynamoDB table
resource "aws_dynamodb_table" "urls" {
  name         = "tui-li-urls"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "pk"

  attribute {
    name = "pk"
    type = "S"
  }

  ttl {
    attribute_name = "ttl"   # the attribute your app will set with an epoch timestamp (in seconds)
    enabled        = true    # turns TTL on
  }

  tags = {
    app         = var.service_name
    environment = "prod"
    managed_by  = "terraform"
  }
}

# 2) Create the Lightsail Container Service
resource "aws_lightsail_container_service" "service" {
  name  = var.service_name
  power = var.power
  scale = var.scale

  public_domain_names {
    certificate {
      certificate_name = var.certificate_name 
      domain_names     = ["tuili.kiwi", "www.tuili.kiwi", "api.tuili.kiwi"]
    }
  }

  tags = {
    app         = var.service_name
    environment = "prod"       # adjust as needed: dev | staging | prod
    managed_by  = "terraform" # helpful for AWS console clarity
  }
}
