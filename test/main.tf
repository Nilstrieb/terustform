terraform {
  required_providers {
    terustform = {
        source = "github.com/Nilstrieb/terustform"
    }
  }
}

provider "terustform" {}

resource "terustform_hello" "test1" {}