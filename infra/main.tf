# 2) Create the DynamoDB table
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

  tags = {
    app         = var.service_name
    environment = "prod"       # adjust as needed: dev | staging | prod
    managed_by  = "terraform" # helpful for AWS console clarity
  }
}

# 2) Deploy using a public image from Docker Hub
resource "aws_lightsail_container_service_deployment_version" "deployment" {
  service_name = aws_lightsail_container_service.service.name

  container {
    container_name = var.container_name
    image          = var.container_image  # e.g. "youruser/tui-li:latest"
    environment = {
      # existing
      HOST                 = "0.0.0.0"
      PORT                 = tostring(var.container_port)
      RUST_LOG             = "info"

      # aws/dynamo config for PROD (no DYNAMODB_ENDPOINT here)
      AWS_REGION           = var.region
      AWS_ACCESS_KEY_ID    = var.app_aws_access_key_id
      AWS_SECRET_ACCESS_KEY= var.app_aws_secret_access_key
      DYNAMODB_TABLE       = aws_dynamodb_table.urls.name
    }
    ports = {
      "${var.container_port}" = "HTTP"
    }
  }

  public_endpoint {
    container_name = var.container_name
    container_port = var.container_port
    health_check {
      path                = var.health_check_path
      success_codes       = "200-399"
      interval_seconds    = 5
      timeout_seconds     = 2
      healthy_threshold   = 2
      unhealthy_threshold = 2
    }
  }
}
