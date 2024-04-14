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
  name = "aa mykitten"
}

data "terustform_kitty" "hellyes" {
  name = "aa a cute kitty"
}

output "meow" {
  value = data.terustform_kitty.kitty.id
}
output "cat1" {
  value = data.terustform_kitty.kitty.meow
}
output "cat2" {
  value = data.terustform_kitty.hellyes.meow
}