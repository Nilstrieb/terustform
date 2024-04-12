terraform {
  required_providers {
    terustform = {
        source = "github.com/Nilstrieb/terustform"
    }
  }
}

provider "terustform" {}

//resource "terustform_hello" "test1" {}

data "terustform_kitty" "kitty" {
  name = "mykitten"
}

data "terustform_kitty" "hellyes" {
  name = "a cute kitty"
}

output "meow" {
  value = data.terustform_kitty.kitty.id
}
output "hellyes" {
  value = data.terustform_kitty.kitty.meow
}