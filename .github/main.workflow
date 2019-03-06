workflow "Main" {
  on = "push"
  resolves = ["Deploy to swarm"]
}

action "Build image" {
  uses = "actions/docker/cli@76ff57a"
  args = "build -t repo.treescale.com/sagebind/fmbq-timer:0.3.0."
}

action "Master" {
  needs = ["Build image"]
  uses = "actions/bin/filter@b2bea07"
  args = "branch master"
}

action "Registry login" {
  needs = ["Master"]
  uses = "actions/docker/login@76ff57a"
  env = {
    DOCKER_REGISTRY_URL = "repo.treescale.com"
  }
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Push image" {
  needs = ["Registry login"]
  uses = "actions/docker/cli@76ff57a"
  args = "push repo.treescale.com/sagebind/fmbq-timer:0.3.0"
}

action "Deploy to swarm" {
  uses = "sagebind/docker-swarm-deploy-action@master"
  needs = ["Push image"]
  secrets = ["DOCKER_REMOTE_HOST", "DOCKER_SSH_PRIVATE_KEY", "DOCKER_SSH_PUBLIC_KEY"]
  args = "stack deploy --with-registry-auth --prune --compose-file deploy/prod.yaml fmbq-timer"
}
