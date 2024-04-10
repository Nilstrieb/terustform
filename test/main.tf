terraform {
  required_providers {
    terustform = {
        source = "github.com/Nilstrieb/terustform"
    }
  }
}

provider "terustform" {}

//resource "terustform_hello" "test1" {}

data "terustform_kitty" "kitty" {}

output "meow" {
  value = data.terustform_kitty.kitty.kitten
}