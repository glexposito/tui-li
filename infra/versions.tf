terraform {
  required_version = ">= 1.5.0"

  cloud {
    organization = "cybertronics"
    workspaces {
      name = "tui-li"
    }
  }
}
