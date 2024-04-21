terraform {
  required_providers {
    corsschool = {
        source = "github.com/Nilstrieb/corsschool"
    }
  }
}

provider "corsschool" {}

//resource "terustform_hello" "test1" {}

data "corsschool_kitty" "kitty" {
  name = "aa mykitten"
}
data "corsschool_kitty" "hellyes" {
  name = "aa a cute kitty"
}
output "cat1" {
  value = data.corsschool_kitty.kitty.meow
}
output "cat2" {
  value = data.corsschool_kitty.hellyes.meow
}

data "corsschool_hugo" "hugo" {}
output "hugo" {
  value = data.corsschool_hugo.hugo
}

data "corsschool_class" "test" {
  id = "f245514b-f99c-4c09-ab53-eabd944af6d2"
}
output "class" {
  value = data.corsschool_class.test
}