# 1) Create the Lightsail Container Service
resource "aws_lightsail_container_service" "service" {
  name  = var.service_name
  power = var.power
  scale = var.scale

  tags = {
    app         = var.service_name
    environment = "dev"       # adjust as needed: dev | staging | prod
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
      HOST     = "0.0.0.0"
      PORT     = tostring(var.container_port)
      RUST_LOG = "info"
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
