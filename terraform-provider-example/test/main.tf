terraform {
  required_providers {
    example = {
        source = "github.com/Nilstrieb/example"
    }
  }
}

provider "example" {}

//resource "terustform_hello" "test1" {}

data "example_kitty" "kitty" {
  name = "aa mykitten"
}
data "example_kitty" "hellyes" {
  name = "aa a cute kitty"
}
output "cat1" {
  value = data.example_kitty.kitty.meow
}
output "cat2" {
  value = data.example_kitty.hellyes.meow
}