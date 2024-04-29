terraform {
  required_providers {
    corsschool = {
        source = "github.com/Nilstrieb/corsschool"
    }
  }
}

provider "corsschool" {}

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

resource "corsschool_class" "myclass" {
  name = "meow"
  description = "???"
}