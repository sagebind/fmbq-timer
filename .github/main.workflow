workflow "main" {
  on = "push"
  resolves = ["deploy"]
}

action "build" {
  uses = "actions/docker/cli@master"
  args = ["build", "-t", "repo.treescale.com/sagebind/fmbq-timer:$GITHUB_SHA", "."]
}

action "registry-login" {
  uses = "actions/docker/login@master"
  env = {
    DOCKER_REGISTRY_URL = "repo.treescale.com"
  }
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "push" {
  needs = ["build", "registry-login"]
  uses = "actions/docker/cli@master"
  args = ["push", "repo.treescale.com/sagebind/fmbq-timer"]
}

action "master" {
  uses = "actions/bin/filter@master"
  args = "branch master"
}

action "deployment-config" {
  uses = "actions/bin/sh@master"
  args = ["sed -i s/:latest/:$GITHUB_SHA/ $GITHUB_WORKSPACE/config/deployment.yaml"]
}

action "kubeconfig" {
  uses = "digitalocean/action-doctl@master"
  secrets = ["DIGITALOCEAN_ACCESS_TOKEN"]
  args = ["kubernetes cluster kubeconfig show nyc1 > $HOME/.kubeconfig"]
}

action "deploy" {
  needs = ["master", "push", "deployment-config", "kubeconfig"]
  uses = "docker://lachlanevenson/k8s-kubectl"
  runs = "sh -l -c"
  args = ["kubectl --kubeconfig=$HOME/.kubeconfig apply -f $GITHUB_WORKSPACE/config/deployment.yaml"]
}
