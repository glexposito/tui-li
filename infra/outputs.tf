output "service_name" {
  value       = aws_lightsail_container_service.service.name
  description = "Lightsail container service name."
}

output "service_url" {
  value       = aws_lightsail_container_service.service.url
  description = "Default public HTTPS URL (works after a deployment with a public endpoint)."
}
