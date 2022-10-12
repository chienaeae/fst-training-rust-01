variable "TAG" {
    default = "develop"
}

variable "REPOSITORY" {
    default = "registry.cellar.fst.network"
}

variable "DEBUG" {
    default = "0"
}

group "default" {
    targets = [
        "debian",
        "scratch"
    ]
}

target "base" {
    args = {
      DEBUG = "${DEBUG}"
    }
    platforms = ["linux/amd64"]
}

target "debian" {
    inherits = ["base"]
    dockerfile = "dev-support/containers/debian/Dockerfile"
    tags = ["${REPOSITORY}/template/rust:${TAG}"]
}

target "scratch" {
    inherits = ["base"]
    dockerfile = "dev-support/containers/scratch/Dockerfile"
    tags = ["${REPOSITORY}/template/rust:${TAG}"]
}
